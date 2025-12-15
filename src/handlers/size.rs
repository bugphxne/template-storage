use actix_web::{HttpResponse, post, web};
use serde::Deserialize;

use crate::constants::UPLOADS_DIR;
use crate::utils::{fs_ops::compute_size, path::safe_join};

#[derive(Deserialize)]
pub struct SizeRequest {
    pub path: String,
}

#[post("/size")]
pub async fn get_size(req: web::Json<SizeRequest>) -> Result<HttpResponse, actix_web::Error> {
    let full = safe_join(std::path::Path::new(UPLOADS_DIR), &req.path)
        .map_err(actix_web::error::ErrorBadRequest)?;

    let size = compute_size(&full)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "bytes": size
    })))
}
