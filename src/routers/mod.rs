mod balances;
mod allocations;
mod utils;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::app_state::AppState;
use crate::utils as base_utils;

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(base_utils::ping))
        .with_state(state.clone())
        .nest("/balances", balances::get_router(state.clone()))
        .nest("/allocations", allocations::get_router(state.clone()))
}
