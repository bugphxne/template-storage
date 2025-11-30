use actix_web::{HttpResponse, post, web};
use serde::Deserialize;
use tokio::fs;

use crate::config::AppConfig;
use crate::utils::path::safe_join;

#[derive(Deserialize)]
pub struct ListRequest {
    pub path: String,
    pub limit: Option<usize>,
}

#[post("/list")]
pub async fn list_files(
    cfg: web::Data<AppConfig>,
    req: web::Json<ListRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let full = safe_join(std::path::Path::new(&cfg.base_dir), &req.path)
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    let mut result = vec![];

    let mut dir = fs::read_dir(&full)
        .await
        .map_err(|e| actix_web::error::ErrorBadRequest(e))?;

    while let Some(entry) = dir.next_entry().await? {
        result.push(entry.file_name().to_string_lossy().to_string());
        if let Some(lim) = req.limit {
            if result.len() >= lim {
                break;
            }
        }
    }

    Ok(HttpResponse::Ok().json(result))
}
