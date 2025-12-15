use actix_web::{HttpResponse, delete, web};
use serde::Deserialize;

use crate::constants::UPLOADS_DIR;
use crate::utils::{fs_ops::delete_recursively, path::safe_join};

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub path: String,
}

#[delete("/delete")]
pub async fn delete_path(req: web::Json<DeleteRequest>) -> Result<HttpResponse, actix_web::Error> {
    let full = safe_join(std::path::Path::new(UPLOADS_DIR), &req.path)
        .map_err(actix_web::error::ErrorBadRequest)?;

    delete_recursively(&full)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"deleted": req.path})))
}
