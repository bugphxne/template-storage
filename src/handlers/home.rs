use actix_web::{HttpResponse, get};

#[get("/")]
pub async fn home() -> HttpResponse {
    let info = r#"{
  "status": "running",
  "endpoints": {
    "POST /api/upload": "Upload files (form-data: path, file)",
    "POST /api/list": "List files (JSON: {path, limit?})",
    "POST /api/size": "Get size (JSON: {path})",
    "DELETE /api/delete": "Delete path (JSON: {path})"
  }
}"#;

    HttpResponse::Ok()
        .content_type("application/json; charset=utf-8")
        .body(info)
}
