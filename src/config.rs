use dotenvy::dotenv;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub domain: String,
    pub port: u16,
    pub base_dir: String,
    pub allow_origin: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            domain: env::var("DOMAIN").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT").unwrap_or_else(|_| "8080".to_string()).parse().unwrap(),
            base_dir: env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage".to_string()),
            allow_origin: env::var("ALLOW_DOMAIN").unwrap_or_else(|_| "*".to_string()),
        }
    }
}
