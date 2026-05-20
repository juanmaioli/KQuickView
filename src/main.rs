mod config;
mod crypto;
mod database;
mod models;

use config::Config;
use crypto::procesa_rx;
use database::{save_event, get_user_name, get_event_description};
use models::Event;
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::{info, error, Level};
use tracing_subscriber::FmtSubscriber;
use chrono::Local;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Iniciando Receptor Smart (Daemon Rust)...");

    let config = Config::from_env();
    
    // Database connection
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    database::setup_database(&pool).await?;

    info!("Conexión a base de datos establecida.");

    // UDP Socket
    let addr = format!("0.0.0.0:{}", config.listen_port);
    let socket = std::sync::Arc::new(UdpSocket::bind(&addr).await?);
    info!("Escuchando en puerto UDP: {}", config.listen_port);

    let config = std::sync::Arc::new(config);
    let mut buf = [0u8; 1024];

    loop {
        match socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                let data = buf[..len].to_vec();
                
                let socket = socket.clone();
                let pool = pool.clone();
                let config = config.clone();

                tokio::spawn(async move {
                    process_packet(data, addr, socket, pool, config).await;
                });
            }
            Err(e) => {
                error!("Error al recibir paquete UDP: {}", e);
                // Un pequeño delay de cortesía para evitar un loop de errores en caliente si ocurre un fallo persistente
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            }
        }
    }
}

async fn process_packet(data: Vec<u8>, addr: SocketAddr, socket: std::sync::Arc<UdpSocket>, pool: sqlx::Pool<sqlx::MySql>, config: std::sync::Arc<Config>) {
    let (decrypted, encrypted) = procesa_rx(data, &config.aes_key);
    
    if decrypted.is_empty() || decrypted[0] != b'@' {
        return;
    }

    // Basic Parsing (Simplified version of ProcesoUDP logic)
    // The VB6 code does: StrBin = Mid(StrBin, 5, Len(StrBin) - 4) ... Mid(StrBin, 2, Len(StrBin) - 2)
    // This part is very specific to the NETIO protocol.
    
    let content = &decrypted[1..]; // Remove @
    if content.len() < 20 { // Check minimum length
        return;
    }

    // Following VB6 logic: 
    // StrBin = Mid(StrBin, 5, Len(StrBin) - 4) -> content[4..]
    // StrBin = Mid(StrBin, 2, Len(StrBin) - 2) -> content[4+1..len-2]
    
    // However, the MkCid sub uses "Cid" which is a string built with CvAsc
    // Let's try to extract the fields directly if they are at fixed positions.
    // Cid = Abonado(4) + " " + Evento(2) + " " + Particion(4) + " " + Zona(2) + " " + ...
    
    // In VB6:
    // Cid = CvAsc(content[1+4]) + CvAsc(content[2+4]) + CvAsc(content[3+4]) + CvAsc(content[4+4])
    // This looks like BCD or similar encoding. 
    // CvAsc(10) -> "0", CvAsc(11) -> "B", etc.
    
    let extract_cv_asc = |b: u8| -> char {
        match b {
            10 => '0',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            n if n < 10 => (b'0' + n) as char,
            _ => (b'0' + (b % 10)) as char, // Fallback
        }
    };

    // Correcting the indices based on VB6: 
    // StrBin = content (without @)
    // StrBin = content[4..]
    // StrBin = StrBin[1..len-2] (which is content[5..])
    
    if content.len() < 21 {
        return;
    }
    
    let base = 5;
    let abonado = format!("{}{}{}{}", 
        extract_cv_asc(content[base]), 
        extract_cv_asc(content[base+1]), 
        extract_cv_asc(content[base+2]), 
        extract_cv_asc(content[base+3])).replace('D', "0");

    let mut evento = format!("{}{}{}{}", 
        extract_cv_asc(content[base+6]), 
        extract_cv_asc(content[base+7]), 
        extract_cv_asc(content[base+8]), 
        extract_cv_asc(content[base+9]));

    // Normalizar evento según VB6:
    // Evento = Replace(Evento, "E", "1")
    // Evento = Replace(Evento, "R", "3")
    evento = evento.replace('E', "1").replace('R', "3");
        
    let particion = format!("{}{}", 
        extract_cv_asc(content[base+10]), 
        extract_cv_asc(content[base+11]));
        
    let mut zona = format!("{}{}{}", 
        extract_cv_asc(content[base+12]), 
        extract_cv_asc(content[base+13]), 
        extract_cv_asc(content[base+14]));

    let mut description;
    let mut alarma_tipo = "Evento".to_string();

    // Mapear reglas especiales de VB6
    match evento.as_str() {
        "0100" | "0500" => {
            description = "REPORTE DE ESTADO TRANSMISOR".to_string();
            zona = "-".to_string();
        }
        "1400" | "1401" | "1409" | "1402" => {
            description = format!("ABRE USUARIO {}", zona);
            if let Some(user_name) = get_user_name(&pool, &abonado, &zona).await {
                description = format!("ABRE {}", user_name);
            }
        }
        "3400" | "3401" | "3409" | "3402" => {
            description = format!("CIERRA USUARIO {}", zona);
            if let Some(user_name) = get_user_name(&pool, &abonado, &zona).await {
                description = format!("CIERRA {}", user_name);
            }
        }
        "3441" => {
            description = format!("ARMADO PARCIAL CIERRA USUARIO {}", zona);
            if let Some(user_name) = get_user_name(&pool, &abonado, &zona).await {
                description = format!("ARMADO PARCIAL CIERRA {}", user_name);
            }
        }
        "3408" => {
            description = "ARMADO RAPIDO".to_string();
        }
        "1301" => {
            description = "PERDIDAD DE 220V".to_string();
            zona = "-".to_string();
            alarma_tipo = "AlarmaN".to_string();
        }
        "3301" => {
            description = "RECONECCION DE 220V".to_string();
            zona = "-".to_string();
        }
        "1302" => {
            description = "DESCONECCION BATERIA/BATERIA BAJA".to_string();
            zona = "-".to_string();
            alarma_tipo = "AlarmaN".to_string();
        }
        "3302" => {
            description = "RECONECCION BATERIA".to_string();
            zona = "-".to_string();
        }
        "1570" => {
            description = "ZONA INHIBIDA POR CENTRAL".to_string();
        }
        "1321" => {
            description = "DESCONECCION SIRENA".to_string();
            zona = "-".to_string();
            alarma_tipo = "AlarmaN".to_string();
        }
        "3321" => {
            description = "RECONECCION SIRENA".to_string();
            zona = "-".to_string();
        }
        "1250" => {
            description = "FALLO DE KEEP ALIVE".to_string();
            zona = "-".to_string();
            alarma_tipo = "AlarmaN".to_string();
        }
        "3250" => {
            description = "RESTAURACION DE KEEP ALIVE".to_string();
            zona = "-".to_string();
        }
        "1139" => {
            description = format!("VERIFICACION DE INTRUSION EN ZONA {}", zona);
            alarma_tipo = "AlarmaN".to_string();
        }
        _ => {
            // Caso general: Reemplazar letras por números (A=999, B=998, C=997...)
            let mut search_event = evento.clone();
            if search_event.contains('A') { search_event = "999".to_string(); }
            else if search_event.contains('B') { search_event = "998".to_string(); }
            else if search_event.contains('C') { search_event = "997".to_string(); }
            else if search_event.contains('D') { search_event = "996".to_string(); }
            else if search_event.contains('E') { search_event = "995".to_string(); }
            else if search_event.contains('F') { search_event = "994".to_string(); }
            else if search_event.contains('G') { search_event = "993".to_string(); }
            else if search_event.contains('H') { search_event = "992".to_string(); }

            let cod = if search_event.len() >= 3 { &search_event[search_event.len()-3..] } else { &search_event };

            let (db_desc, db_alarma) = get_event_description(&pool, cod, &evento).await;
            description = db_desc;
            alarma_tipo = db_alarma;
        }
    }

    let event = Event {
        abonado,
        fecha: Local::now(),
        evento,
        zona,
        particion,
        descripcion: description,
        alarma_tipo,
    };

    if let Err(e) = save_event(&pool, &event).await {
        error!("Error guardando evento: {}", e);
    }

    // ACK
    let ack_bits = 0u8;
    let ack_raw = vec![b'@', 0x30 + ack_bits];
    let ack = crypto::procesa_tx(ack_raw, &config.aes_key, encrypted);
    
    if let Err(e) = socket.send_to(&ack, addr).await {
        error!("Error enviando ACK: {}", e);
    }
}
