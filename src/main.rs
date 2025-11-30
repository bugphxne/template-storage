mod config;
mod handlers;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_web::{App, HttpServer};
use config::AppConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = AppConfig::load();

    std::fs::create_dir_all(&cfg.base_dir)?;

    println!("Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.allow_origin)
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allow_any_header();

        App::new()
            .app_data(actix_web::web::Data::new(cfg.clone()))
            .wrap(cors)
            .configure(routes::api)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
