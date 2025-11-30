use actix_web::{HttpResponse, delete, web};
use serde::Deserialize;

use crate::config::AppConfig;
use crate::utils::{fs_ops::delete_recursively, path::safe_join};

#[derive(Deserialize)]
pub struct DeleteRequest {
    pub path: String,
}

#[delete("/delete")]
pub async fn delete_path(
    cfg: web::Data<AppConfig>,
    req: web::Json<DeleteRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let full = safe_join(std::path::Path::new(&cfg.base_dir), &req.path)
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    delete_recursively(&full)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"deleted": req.path})))
}
