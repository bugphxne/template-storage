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

    println!("Server running at http://{}:{}", cfg.domain, cfg.port);

    let server_cfg = cfg.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&server_cfg.allow_origin)
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allow_any_header();

        App::new()
            .app_data(actix_web::web::Data::new(server_cfg.clone()))
            .wrap(cors)
            .configure(routes::api)
    })
    .bind((cfg.domain, cfg.port))?
    .run()
    .await
}
