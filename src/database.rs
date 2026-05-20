use sqlx::{MySql, Pool, Row};
use crate::models::Event;
use chrono::{Datelike, Local};
use tracing::info;

pub async fn setup_database(_pool: &Pool<MySql>) -> Result<(), sqlx::Error> {
    // Base tables setup logic could go here
    info!("Base de datos configurada.");
    Ok(())
}

pub async fn get_monthly_table_name() -> String {
    let now = Local::now();
    format!("rep_{}{}", now.year(), now.month())
}

pub async fn ensure_monthly_table(pool: &Pool<MySql>, table_name: &str) -> Result<(), sqlx::Error> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS `{}` (
            `rep_id` int(11) NOT NULL AUTO_INCREMENT,
            `rep_abonado` varchar(10) DEFAULT NULL,
            `rep_fec` datetime DEFAULT NULL,
            `rep_cadena` varchar(50) DEFAULT NULL,
            `rep_evento` varchar(100) DEFAULT NULL,
            `rep_evento_cod` varchar(10) DEFAULT NULL,
            `rep_zona` varchar(10) DEFAULT NULL,
            `rep_particion` varchar(10) DEFAULT NULL,
            `ab_seg` varchar(2) DEFAULT NULL,
            PRIMARY KEY (`rep_id`),
            KEY `idx_abonado` (`rep_abonado`),
            KEY `idx_fecha` (`rep_fec`)
        ) ENGINE=InnoDB DEFAULT CHARSET=utf8;",
        table_name
    );
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn save_event(pool: &Pool<MySql>, event: &Event) -> Result<(), sqlx::Error> {
    let table_name = get_monthly_table_name().await;
    ensure_monthly_table(pool, &table_name).await?;

    let fecha_mysql = event.fecha.format("%Y-%m-%d %H:%M:%S").to_string();
    let ab_seg = event.fecha.format("%S").to_string();

    // Insert into monthly table
    let insert_query = format!(
        "INSERT INTO `{}` (rep_abonado, rep_fec, rep_cadena, rep_evento, rep_evento_cod, rep_zona, rep_particion, ab_seg) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        table_name
    );

    sqlx::query(&insert_query)
        .bind(&event.abonado)
        .bind(&fecha_mysql)
        .bind(&event.alarma_tipo)
        .bind(&event.descripcion)
        .bind(&event.evento)
        .bind(&event.zona)
        .bind(&event.particion)
        .bind(&ab_seg)
        .execute(pool)
        .await?;

    // Update abonados
    let mut sql_abonado = String::new();
    if event.evento == "3400" || event.evento == "3401" || event.evento == "3409" || event.evento == "3402" || event.evento == "3408" {
        sql_abonado = format!("Update abonados set ab_armado = 'CERRADO', ab_fecultrepo = '{}' where ad_idname = '{}'", fecha_mysql, event.abonado);
    } else if event.evento == "1400" || event.evento == "1401" || event.evento == "1409" || event.evento == "1402" {
        sql_abonado = format!("Update abonados set ab_armado = 'ABIERTO', ab_fecultrepo = '{}' where ad_idname = '{}'", fecha_mysql, event.abonado);
    } else if event.evento == "3441" {
        sql_abonado = format!("Update abonados set ab_armado = 'CERRADO PARCIAL', ab_fecultrepo = '{}' where ad_idname = '{}'", fecha_mysql, event.abonado);
    }

    if !sql_abonado.is_empty() {
        sqlx::query(&sql_abonado).execute(pool).await?;
    }

    // Always update last report date
    sqlx::query("UPDATE abonados SET ab_lr = ? WHERE ad_idname = ?")
        .bind(&fecha_mysql)
        .bind(&event.abonado)
        .execute(pool)
        .await?;

    // Update rep_last
    sqlx::query("TRUNCATE rep_last").execute(pool).await?;
    sqlx::query("INSERT INTO rep_last(rep_abonado, rep_fec, rep_cadena, rep_evento, rep_evento_cod, rep_zona, rep_particion, ab_seg) VALUES(?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&event.abonado)
        .bind(&fecha_mysql)
        .bind(&event.alarma_tipo)
        .bind(&event.descripcion)
        .bind(&event.evento)
        .bind(&event.zona)
        .bind(&event.particion)
        .bind(&ab_seg)
        .execute(pool)
        .await?;

    info!("Evento guardado para abonado: {}", event.abonado);
    Ok(())
}

pub async fn get_user_name(pool: &Pool<MySql>, abonado: &str, zona: &str) -> Option<String> {
    let row = sqlx::query("SELECT user_nombre From usuarios WHERE ABONADO_ID = ? and user_orden = ?")
        .bind(abonado)
        .bind(zona)
        .fetch_optional(pool)
        .await
        .ok()??;
    
    Some(row.get(0))
}

pub async fn get_event_description(pool: &Pool<MySql>, cod: &str, original_evento: &str) -> (String, String) {
    let row = sqlx::query("SELECT desc_spa, categoria, cat_nombre From rep_cotactid WHERE cod = ?")
        .bind(cod)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten();

    if let Some(r) = row {
        let desc: String = r.get("desc_spa");
        let cat: String = r.get("categoria");
        let cat_nombre: String = r.get("cat_nombre");
        
        let alarma_tipo = match cat_nombre.as_str() {
            "100" | "110" | "120" | "130" => "AlarmaN".to_string(),
            _ => "Evento".to_string(),
        };

        let prefix = if original_evento.starts_with('3') {
            "REESTABLECE-->"
        } else {
            ""
        };

        (format!("{}{}:{}", prefix, cat, desc), alarma_tipo)
    } else {
        (original_evento.to_string(), "Evento".to_string())
    }
}
