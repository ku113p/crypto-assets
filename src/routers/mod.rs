mod balances;
mod allocations;
pub(crate) mod utils;
pub mod index;
mod views;

use std::sync::Arc;
use axum::Router;
use crate::app_state::AppState;

type TRouter = Router<Arc<AppState>>;

pub fn get_router(state: Arc<AppState>) -> Router {
    Router::new()
        .nest("/v1/balances", balances::Api::REST.get_router(state.clone()))
        .nest("/v1-htmx/balances", balances::Api::HTMX.get_router(state.clone()))
        .nest("/v1/allocations", allocations::Api::REST.get_router(state.clone()))
        .nest("/v1-htmx/allocations", allocations::Api::HTMX.get_router(state.clone()))
        .nest("/v1/views", views::Api::REST.get_router(state.clone()))
        .nest("/v1-htmx/views", views::Api::HTMX.get_router(state.clone()))
}
