use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};

pub fn get_success_response(status: StatusCode) -> (StatusCode, Json<Value>) {
    get_response(status, json!({ "success": true }))
}

pub fn get_response(status: StatusCode, value: Value) -> (StatusCode, Json<Value>) {
    (status, Json(value))
}