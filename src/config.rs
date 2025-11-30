#[derive(Clone)]
pub struct AppConfig {
    pub base_dir: String,
    pub allow_origin: String,
}

impl AppConfig {
    pub fn load() -> Self {
        Self {
            base_dir: "./storage".to_string(),
            allow_origin: "*".to_string(),
        }
    }
}
