use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub base_url: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv::dotenv().ok();
    Config {
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        redis_url: std::env::var("REDIS_URL").expect("REDIS_URL must be set"),
        server_host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
        server_port: std::env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .expect("SERVER_PORT must be a number"),
        base_url: std::env::var("BASE_URL").unwrap_or_else(|_| "https://link.aescipher.xyz".to_string()),
    }
}); 