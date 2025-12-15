use actix_web::{HttpResponse, post, web};
use serde::Deserialize;
use std::path::Path;
use tokio::fs;

use crate::constants::UPLOADS_DIR;
use crate::utils::path::safe_join;

#[derive(Deserialize)]
pub struct ListRequest {
    pub path: String,
    pub limit: Option<usize>,
}

#[post("/list")]
pub async fn list_files(req: web::Json<ListRequest>) -> Result<HttpResponse, actix_web::Error> {
    let full =
        safe_join(Path::new(UPLOADS_DIR), &req.path).map_err(actix_web::error::ErrorBadRequest)?;

    let mut result = Vec::new();
    let mut dir = fs::read_dir(&full)
        .await
        .map_err(actix_web::error::ErrorBadRequest)?;

    while let Some(entry) = dir.next_entry().await? {
        result.push(entry.file_name().to_string_lossy().to_string());

        if let Some(limit) = req.limit {
            if result.len() >= limit {
                break;
            }
        }
    }

    Ok(HttpResponse::Ok().json(result))
}
