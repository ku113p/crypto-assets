use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::app_state::AppState;
use crate::models::models::Balance;
use crate::utils;

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(utils::ping))
        .route(
            "/", get(list)
                .post(create_or_update),
        )
        .route("/{symbol}", delete(remove))
        .with_state(state.clone())
}

#[derive(Serialize, Deserialize)]
struct BalanceResponse {
    symbol: String,
    amount: f32,
}

impl BalanceResponse {
    fn new(symbol: String, amount: f32) -> Self {
        Self { symbol, amount }
    }
}

async fn list(State(app_state): State<Arc<AppState>>) -> Json<Vec<BalanceResponse>> {
    let storage = app_state.storage.lock().await;
    let balances: Vec<BalanceResponse> = storage.balances.iter()
        .filter_map(|b| {
            storage.tokens
                .get(&b.token_id)
                .map(|token| BalanceResponse::new(token.symbol.clone(), b.amount))
        })
        .collect();
    Json(balances)
}

#[derive(Serialize, Deserialize)]
struct BalanceRequest {
    symbol: String,
    amount: f32,
}

async fn create_or_update(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<BalanceRequest>,
) -> impl IntoResponse {
    let mut storage = app_state.storage.lock().await;

    let token_id = storage.get_or_create_token_id(&request.symbol);

    let created = match storage.balances.iter_mut()
        .find(|b| b.token_id == token_id) {
        None => {
            storage.balances.push(Balance::new(token_id, request.amount));
            true
        }
        Some(existing) => {
            existing.amount = request.amount;
            false
        }
    };

    let status = if created { StatusCode::CREATED } else { StatusCode::OK };

    get_success_response(status)
}

fn get_success_response(status: StatusCode) -> (StatusCode, Json<Value>) {
    get_response(status, json!({ "success": true }))
}

fn get_response(status: StatusCode, value: Value) -> (StatusCode, Json<Value>) {
    (status, Json(value))
}

async fn remove(
    State(app_state): State<Arc<AppState>>,
    Path(symbol): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut storage = app_state.storage.lock().await;

    if let Some(token_id) = storage.get_token_id(&symbol) {
        storage.balances = storage.balances.iter()
            .filter(|b| b.token_id != token_id)
            .cloned()
            .collect();

        return Err(StatusCode::NO_CONTENT);
    }

    Ok(get_response(StatusCode::NOT_FOUND, json!({"message": "not_found"})))
}
