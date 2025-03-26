use std::sync::Arc;
use axum::extract::{Path, State};
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::app_state::AppState;
use crate::routers::allocations::methods::{AllocationStore, AllocationView};
use crate::routers::utils;

pub struct Methods;

impl Methods {
    pub async fn list(State(app_state): State<Arc<AppState>>) -> Json<Vec<AllocationView>> {
        let allocations = AllocationStore::new(app_state).list().await.into();
        Json(allocations)
    }

    pub async fn create_or_update(
        State(app_state): State<Arc<AppState>>,
        Json(allocation): Json<AllocationView>,
    ) -> impl IntoResponse {
        let created = AllocationStore::new(app_state).create_or_update(allocation).await;

        let status = if created { StatusCode::CREATED } else { StatusCode::OK };
        utils::get_success_response(status)
    }

    pub async fn remove(
        State(app_state): State<Arc<AppState>>,
        Path((scheme_name, symbol)): Path<(String, String)>
    ) -> Result<impl IntoResponse, StatusCode> {
        match AllocationStore::new(app_state).remove(scheme_name, symbol).await {
            true => Err(StatusCode::NO_CONTENT),
            false => Ok(utils::get_not_found_response())
        }
    }
}