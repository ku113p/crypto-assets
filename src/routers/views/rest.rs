use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Json};
use crate::app_state::AppState;
use crate::auth::AuthToken;
use crate::routers::views::methods::{TokenInfo, Views};

pub struct Methods;

impl Methods {
    pub async fn info(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
    ) -> Json<Vec<TokenInfo>> {
        let token_infos = Views::new(app_state, auth_token.0).info().await;
        Json(token_infos)
    }
}
