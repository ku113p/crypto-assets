use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use tokio::fs;

type Response = (StatusCode, Json<Value>);

pub fn get_success_response(status: StatusCode) -> Response {
    get_response(status, json!({ "success": true }))
}

pub fn get_response(status: StatusCode, value: Value) -> Response {
    (status, Json(value))
}

pub fn get_not_found_response() -> Response {
    get_response(StatusCode::NOT_FOUND, json!({"message": "not_found"}))
}

pub async fn get_file_text(filename: &str) -> String {
    fs::read_to_string(format!("templates/{filename}"))
        .await
        .unwrap_or_else(|_| "<h1>Error: HTML file not found</h1>".to_string())
}
