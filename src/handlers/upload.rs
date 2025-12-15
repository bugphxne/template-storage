use actix_multipart::Multipart;
use actix_web::{HttpResponse, post};
use futures_util::StreamExt;
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::constants::UPLOADS_DIR;
use crate::utils::path::{get_relative_path, safe_join};

#[post("/upload")]
pub async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let base = Path::new(UPLOADS_DIR);
    let mut target_path = String::new();
    let mut saved_files = Vec::new();

    while let Some(field) = payload.next().await {
        let mut field = field?;
        let name = field.content_disposition().get_name().unwrap_or("");

        match name {
            "path" => {
                target_path = read_field_as_string(&mut field).await?;
            }
            "file" => {
                let file_info = save_uploaded_file(base, &target_path, &mut field).await?;
                saved_files.push(file_info);
            }
            _ => {
                consume_field(&mut field).await?;
            }
        }
    }

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "saved": saved_files,
    })))
}

async fn read_field_as_string(
    field: &mut actix_multipart::Field,
) -> Result<String, actix_web::Error> {
    let mut buf = Vec::new();
    while let Some(chunk) = field.next().await {
        buf.extend_from_slice(&chunk?);
    }
    String::from_utf8(buf)
        .map(|s| s.trim().to_string())
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid UTF-8 in path"))
}

async fn save_uploaded_file(
    base: &Path,
    rel_path: &str,
    field: &mut actix_multipart::Field,
) -> Result<serde_json::Value, actix_web::Error> {
    let safe_dir = safe_join(base, rel_path).map_err(actix_web::error::ErrorBadRequest)?;

    fs::create_dir_all(&safe_dir)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("mkdir: {}", e)))?;

    let filename = generate_filename(field);
    let fullpath = safe_dir.join(&filename);

    let mut file = fs::File::create(&fullpath)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(format!("create file: {}", e)))?;

    while let Some(chunk) = field.next().await {
        file.write_all(&chunk?).await.map_err(|e| {
            actix_web::error::ErrorInternalServerError(format!("write file: {}", e))
        })?;
    }

    let uuid = filename.split('.').next().unwrap_or(&filename);
    Ok(serde_json::json!({
        "id": uuid,
        "path": get_relative_path(base, &fullpath)
    }))
}

fn generate_filename(field: &actix_multipart::Field) -> String {
    let uuid = Uuid::new_v4();
    let ext = field
        .content_disposition()
        .get_filename()
        .and_then(|f| Path::new(f).extension())
        .and_then(|e| e.to_str())
        .unwrap_or("");

    if ext.is_empty() {
        uuid.to_string()
    } else {
        format!("{}.{}", uuid, ext)
    }
}

async fn consume_field(field: &mut actix_multipart::Field) -> Result<(), actix_web::Error> {
    while let Some(chunk) = field.next().await {
        chunk?;
    }
    Ok(())
}
