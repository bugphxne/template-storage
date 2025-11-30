use actix_multipart::Multipart;
use actix_web::{HttpResponse, post, web};
use futures_util::StreamExt;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::config::AppConfig;
use crate::utils::path::{get_relative_path, safe_join};

#[post("/upload")]
pub async fn upload_file(
    cfg: web::Data<AppConfig>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    let base = std::path::Path::new(&cfg.base_dir);

    let mut target_path: Option<String> = None;
    let mut saved_files: Vec<serde_json::Value> = vec![];

    while let Some(item) = payload.next().await {
        let mut field = item?;
        let content_disposition = field.content_disposition();
        let name = content_disposition.get_name().unwrap_or("");

        match name {
            "path" => {
                let mut buf = Vec::new();
                while let Some(chunk) = field.next().await {
                    buf.extend_from_slice(&chunk?);
                }

                if let Ok(path_str) = String::from_utf8(buf) {
                    target_path = Some(path_str.trim().to_string());
                }
            }

            "file" => {
                let rel = target_path.clone().unwrap_or_default();

                let safe_dir =
                    safe_join(base, &rel).map_err(|e| actix_web::error::ErrorBadRequest(e))?;

                fs::create_dir_all(&safe_dir).await.map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("mkdir: {}", e))
                })?;

                let uuid = Uuid::new_v4();
                let ext = content_disposition
                    .get_filename()
                    .and_then(|f| std::path::Path::new(f).extension())
                    .and_then(|e| e.to_str())
                    .unwrap_or("");

                let filename = if ext.is_empty() {
                    uuid.to_string()
                } else {
                    format!("{}.{}", uuid, ext)
                };

                let fullpath = safe_dir.join(&filename);

                let mut file = fs::File::create(&fullpath).await.map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!("create file: {}", e))
                })?;

                while let Some(chunk) = field.next().await {
                    file.write_all(&chunk?).await.map_err(|e| {
                        actix_web::error::ErrorInternalServerError(format!("write file: {}", e))
                    })?;
                }

                saved_files.push(serde_json::json!({
                    "id": uuid.to_string(),
                    "path": get_relative_path(base, &fullpath)
                }));
            }

            _ => {
                while let Some(chunk) = field.next().await {
                    let _ = chunk?;
                }
            }
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "saved": saved_files,
    })))
}
