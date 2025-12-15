use dotenvy::dotenv;
use std::env;
use crate::constants::*;

#[derive(Clone)]
pub struct AppConfig {
    pub domain: String,
    pub port: u16,
    pub allow_origin: String,
}

impl AppConfig {
    pub fn load() -> Self {
        dotenv().ok();

        Self {
            domain: env::var("DOMAIN").unwrap_or_else(|_| DEFAULT_DOMAIN.to_string()),
            port: env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DEFAULT_PORT),
            allow_origin: env::var("ALLOW_DOMAIN")
                .unwrap_or_else(|_| DEFAULT_ALLOW_ORIGIN.to_string()),
        }
    }
}
