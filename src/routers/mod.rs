mod balances;
mod allocations;
mod utils;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::app_state::AppState;
use crate::utils as base_utils;

type TRouter = Router<Arc<AppState>>;

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ping", get(base_utils::ping))
        .nest("/v1/balances", balances::Api::REST.get_router(state.clone()))
        .nest("/v1-htmx/balances", balances::Api::HTMX.get_router(state.clone()))
        .nest("/v1/allocations", allocations::Api::REST.get_router(state.clone()))
        .nest("/v1-htmx/allocations", allocations::Api::HTMX.get_router(state.clone()))
}
