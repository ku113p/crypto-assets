use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::app_state::AppState;
use crate::models::models::{Allocation};
use crate::routers::utils;
use crate::utils as base_utils;

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(base_utils::ping))
        .route("/", get(list).post(create_or_update))
        .route("/{scheme_name}/{symbol}", delete(remove))
        .with_state(state.clone())
}

#[derive(Serialize, Deserialize)]
struct AllocationResponse {
    scheme_name: String,
    symbol: String,
    amount: f32,
}

impl AllocationResponse {
    fn new(scheme_name: String, symbol: String, amount: f32) -> Self {
        Self { scheme_name, symbol, amount }
    }
}

async fn list(State(app_state): State<Arc<AppState>>) -> Json<Vec<AllocationResponse>> {
    let storage = app_state.storage.lock().await;
    let allocations: Vec<AllocationResponse> = storage.allocations.iter()
        .filter_map(|a| {
            let token = storage.tokens.get(&a.token_id)?;
            let scheme = storage.schemes.get(&a.scheme_id)?;
            Some(AllocationResponse::new(scheme.name.clone(), token.symbol.clone(), a.amount))
        })
        .collect();
    Json(allocations)
}

#[derive(Serialize, Deserialize)]
struct AllocationRequest {
    scheme_name: String,
    symbol: String,
    amount: f32,
}

async fn create_or_update(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<AllocationRequest>,
) -> impl IntoResponse {
    let mut storage = app_state.storage.lock().await;

    let token_id = storage.get_or_create_token_id(&request.symbol);
    let scheme_id = storage.get_or_create_scheme_id(&request.scheme_name);

    let created = match storage.allocations.iter_mut()
        .find(|a| a.token_id == token_id && a.scheme_id == scheme_id) {
        None => {
            storage.allocations.push(Allocation::new(token_id, scheme_id, request.amount));
            true
        }
        Some(existing) => {
            existing.amount = request.amount;
            false
        }
    };

    let status = if created { StatusCode::CREATED } else { StatusCode::OK };
    utils::get_success_response(status)
}

async fn remove(
    State(app_state): State<Arc<AppState>>,
    Path((scheme_name, symbol)): Path<(String, String)>
) -> Result<impl IntoResponse, StatusCode> {
    let mut storage = app_state.storage.lock().await;

    if let Some(scheme_id) = storage.get_scheme_id(&scheme_name) {
        if let Some(token_id) = storage.get_token_id(&symbol){
            storage.allocations = storage.allocations.iter()
                .filter(|a| !(a.token_id == token_id && a.scheme_id == scheme_id))
                .cloned()
                .collect();

            return Err(StatusCode::NO_CONTENT);
        }
    };

    Ok(utils::get_response(StatusCode::NOT_FOUND, json!({"message": "not_found"})))
}
