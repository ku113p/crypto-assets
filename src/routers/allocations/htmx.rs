use std::sync::Arc;
use axum::{extract::State, Form};
use axum::extract::Path;
use axum::response::Html;
use serde::Deserialize;
use crate::app_state::AppState;
use crate::routers::allocations::methods::{AllocationStore, AllocationView};

pub struct Methods;

impl Methods {
    pub async fn list(State(app_state): State<Arc<AppState>>) -> Html<String> {
        let allocations = AllocationStore::new(app_state).list().await;
        let html = allocations.into_iter().map(|a| {
            format!(
                "<tr id='allocation-{}-{}'><td>{}</td><td>{}</td><td>{}</td><td><button hx-delete='/allocations/{}/{}' hx-target='#allocation-{}-{}'>Delete</button></td></tr>",
                a.scheme_name, a.symbol, a.scheme_name, a.symbol, a.amount, a.scheme_name, a.symbol, a.scheme_name, a.symbol
            )
        }).collect::<Vec<_>>().join("\n");

        Html(format!("<table><tr><th>Scheme</th><th>Symbol</th><th>Amount</th><th>Actions</th></tr>{}</table>", html))
    }

    pub async fn create_or_update(
        State(app_state): State<Arc<AppState>>,
        Form(allocation): Form<AllocationView>
    ) -> Html<String> {
        let _created = AllocationStore::new(app_state).create_or_update(allocation.clone()).await;

        Html(format!(
            "<tr id='allocation-{}-{}'><td>{}</td><td>{}</td><td>{}</td><td><button hx-delete='/allocations/{}/{}' hx-target='#allocation-{}-{}'>Delete</button></td></tr>",
            allocation.scheme_name, allocation.symbol, allocation.scheme_name, allocation.symbol, allocation.amount, allocation.scheme_name, allocation.symbol, allocation.scheme_name, allocation.symbol
        ))
    }

    pub async fn remove(
        State(app_state): State<Arc<AppState>>,
        Path(form): Path<RemoveRequest>
    ) -> Html<String> {
        match AllocationStore::new(app_state).remove(form.scheme_name.clone(), form.symbol.clone()).await {
            true => Html(format!("<tr id='allocation-{}-{}' hx-swap-oob='delete'></tr>", form.scheme_name, form.symbol)),
            false => Html("Not Found".to_string()),
        }
    }
}

#[derive(Deserialize)]
pub struct RemoveRequest {
    scheme_name: String,
    symbol: String,
}
