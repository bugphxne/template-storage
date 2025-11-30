use actix_web::{delete, web, HttpResponse};
use serde::Deserialize;

use crate::utils::{path::safe_join, fs_ops::delete_recursively};
use crate::config::AppConfig;

#[derive(Deserialize)]
pub struct DeleteQuery {
    pub path: String,
}

#[delete("/delete")]
pub async fn delete_path(
    cfg: web::Data<AppConfig>,
    q: web::Query<DeleteQuery>
) -> Result<HttpResponse, actix_web::Error> {

    let full = safe_join(std::path::Path::new(&cfg.base_dir), &q.path)
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    delete_recursively(&full)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({"deleted": q.path})))
}
