use std::sync::Arc;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;
use crate::app_state::AppState;

pub async fn ping() -> impl IntoResponse {
    "pong"
}

pub async fn status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let price = state.price_status.lock().await;
    let storage = state.storage.lock().await;
    let workspace_count = storage.workspaces.len();
    let total_tokens: usize = storage.workspaces.values()
        .map(|ws| ws.tokens.len())
        .sum();

    Json(json!({
        "price_worker": {
            "last_result": price.last_result,
            "last_updated": price.last_updated,
            "tokens_updated": price.tokens_updated,
        },
        "workspaces": workspace_count,
        "total_tokens": total_tokens,
    }))
}
