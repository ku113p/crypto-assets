use std::sync::Arc;
use axum::{extract::State, Form};
use axum::extract::Path;
use axum::response::Html;
use serde::Deserialize;
use crate::app_state::AppState;
use crate::routers::balances::methods::{BalanceStore, BalanceView};

pub struct Methods;

impl Methods {
    pub async fn list(State(app_state): State<Arc<AppState>>) -> Html<String> {
        let balances = BalanceStore::new(app_state).list().await;
        let html = balances.into_iter().map(|b| {
            format!(
                "<tr id='balance-{}'><td>{}</td><td>{}</td><td><button hx-delete='/balances/{}' hx-target='#balance-{}'>Delete</button></td></tr>",
                b.symbol, b.symbol, b.amount, b.symbol, b.symbol
            )
        }).collect::<Vec<_>>().join("\n");

        Html(format!("<table><tr><th>Symbol</th><th>Amount</th><th>Actions</th></tr>{}</table>", html))
    }

    pub async fn create_or_update(
        State(app_state): State<Arc<AppState>>,
        Form(balance): Form<BalanceView>
    ) -> Html<String> {
        let _created = BalanceStore::new(app_state).create_or_update(balance.clone()).await;

        Html(format!(
            "<tr id='balance-{}'><td>{}</td><td>{}</td><td><button hx-delete='/balances/{}' hx-target='#balance-{}'>Delete</button></td></tr>",
            balance.symbol, balance.symbol, balance.amount, balance.symbol, balance.symbol
        ))
    }

    pub async fn remove(
        State(app_state): State<Arc<AppState>>,
        Path(form): Path<RemoveRequest>
    ) -> Html<String> {
        match BalanceStore::new(app_state).remove(form.symbol.clone()).await {
            true => Html(format!("<tr id='balance-{}' hx-swap-oob='delete'></tr>", form.symbol)),
            false => Html("Not Found".to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct RemoveRequest {
    symbol: String,
}
