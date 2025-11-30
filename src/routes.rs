use actix_web::web;

use crate::handlers::{delete, home, list, size, upload};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(home::home).service(
        web::scope("/api")
            .service(upload::upload_file)
            .service(delete::delete_path)
            .service(list::list_files)
            .service(size::get_size),
    );
}
