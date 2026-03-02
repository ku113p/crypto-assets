use std::sync::Arc;
use axum::{extract::State, Extension, Form, extract::Path, response::Html};
use serde::Deserialize;
use crate::app_state::AppState;
use crate::auth::AuthToken;
use crate::routers::balances::methods::{BalanceStore, BalanceView};
use crate::routers::utils;

pub struct Methods;

impl Methods {
    pub async fn list(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
    ) -> Html<String> {
        let token = &auth_token.0;
        let balances = BalanceStore::new(app_state, token.clone()).list().await;

        let row_template = utils::get_file_text("row_balance.html").await;
        let rows_html = balances.into_iter()
            .map(|b| row_template
                .replace("{auth_token}", token)
                .replace("{symbol}", &b.symbol)
                .replace("{amount}", &b.amount.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        let list_template = utils::get_file_text("list_balances.html").await;
        let list_html = list_template
            .replace("{auth_token}", token)
            .replace("{rows}", &rows_html);

        Html(list_html)
    }

    pub async fn create_or_update(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
        Form(balance): Form<BalanceView>,
    ) -> Html<String> {
        let token = &auth_token.0;
        let _created = BalanceStore::new(app_state, token.clone()).create_or_update(balance.clone()).await;

        let row_template = utils::get_file_text("row_balance.html").await;
        let row_html = row_template
            .replace("{auth_token}", token)
            .replace("{symbol}", &balance.symbol)
            .replace("{amount}", &balance.amount.to_string());

        Html(row_html)
    }

    pub async fn remove(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
        Path(form): Path<RemoveRequest>,
    ) -> Html<String> {
        match BalanceStore::new(app_state, auth_token.0).remove(form.symbol.clone()).await {
            true => Html(format!("<tr id='balance-{}' hx-swap-oob='delete'></tr>", form.symbol)),
            false => Html("Not Found".to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct RemoveRequest {
    symbol: String,
}
