use actix_web::web;

use crate::handlers::{upload, delete, list, size};

pub fn api(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(upload::upload_file)
            .service(delete::delete_path)
            .service(list::list_files)
            .service(size::get_size)
    );
}
