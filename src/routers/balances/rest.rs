use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::app_state::AppState;
use crate::auth::AuthToken;
use crate::routers::balances::methods::{BalanceStore, BalanceView};
use crate::routers::utils;

pub struct Methods;

impl Methods {
    pub async fn list(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
    ) -> Json<Vec<BalanceView>> {
        let balances = BalanceStore::new(app_state, auth_token.0).list().await;
        Json(balances)
    }

    pub async fn create_or_update(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
        Json(balance): Json<BalanceView>,
    ) -> impl IntoResponse {
        let created = BalanceStore::new(app_state, auth_token.0).create_or_update(balance).await;

        let status = if created { StatusCode::CREATED } else { StatusCode::OK };

        utils::get_success_response(status)
    }

    pub async fn remove(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
        Path(symbol): Path<String>,
    ) -> Result<impl IntoResponse, StatusCode> {
        match BalanceStore::new(app_state, auth_token.0).remove(symbol).await {
            true => Err(StatusCode::NO_CONTENT),
            false => Ok(utils::get_not_found_response())
        }
    }
}
