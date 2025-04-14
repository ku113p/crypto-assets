use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use crate::app_state::AppState;
use crate::routers::views::methods::{TokenInfo, Views};

pub struct Methods;

impl Methods {
    pub async fn info(State(app_state): State<Arc<AppState>>) -> Json<Vec<TokenInfo>> {
        let token_infos = Views::new(app_state).info().await;
        Json(token_infos)
    }
}