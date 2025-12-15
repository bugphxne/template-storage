mod config;
mod constants;
mod handlers;
mod routes;
mod utils;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{App, HttpServer};
use config::AppConfig;
use constants::UPLOADS_DIR;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = AppConfig::load();

    std::fs::create_dir_all(UPLOADS_DIR)?;

    println!("Server running at http://{}:{}", cfg.domain, cfg.port);
    println!(
        "Public files accessible at: http://{}:{}/uploads/",
        cfg.domain, cfg.port
    );

    let server_cfg = cfg.clone();
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&server_cfg.allow_origin)
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allow_any_header();

        App::new()
            .wrap(cors)
            .service(
                Files::new("/uploads", UPLOADS_DIR)
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .configure(routes::api)
    })
    .bind((cfg.domain, cfg.port))?
    .run()
    .await
}
