use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub listen_port: u16,
    pub aes_key: Vec<u8>,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let listen_port = env::var("LISTEN_PORT")
            .unwrap_or_else(|_| "4040".to_string())
            .parse()
            .expect("LISTEN_PORT must be a number");
        
        let aes_key_hex = env::var("AES_KEY").expect("AES_KEY must be set");
        let aes_key = hex::decode(aes_key_hex).expect("AES_KEY must be a valid hex string");

        Config {
            database_url,
            listen_port,
            aes_key,
        }
    }
}
