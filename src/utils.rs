use axum::response::IntoResponse;

pub async fn ping() -> impl IntoResponse {
    "pong"
}