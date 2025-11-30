use actix_web::{HttpResponse, get, web};
use serde::Deserialize;

use crate::config::AppConfig;
use crate::utils::{fs_ops::compute_size, path::safe_join};

#[derive(Deserialize)]
pub struct SizeQuery {
    pub path: String,
}

#[get("/size")]
pub async fn get_size(
    cfg: web::Data<AppConfig>,
    q: web::Query<SizeQuery>,
) -> Result<HttpResponse, actix_web::Error> {
    let full = safe_join(std::path::Path::new(&cfg.base_dir), &q.path)
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    let size = compute_size(&full)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "bytes": size
    })))
}
