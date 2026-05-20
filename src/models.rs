use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct Event {
    pub abonado: String,
    pub fecha: DateTime<Local>,
    pub evento: String,
    pub zona: String,
    pub particion: String,
    pub descripcion: String,
    pub alarma_tipo: String, // "Evento" or "AlarmaN"
}
