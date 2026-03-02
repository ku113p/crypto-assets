use std::sync::Arc;
use axum::{extract::State, Extension, Form};
use axum::extract::Path;
use axum::response::Html;
use serde::Deserialize;
use crate::app_state::AppState;
use crate::auth::AuthToken;
use crate::routers::allocations::methods::{AllocationStore, AllocationView};
use crate::routers::utils;

pub struct Methods;

impl Methods {
    pub async fn list(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
    ) -> Html<String> {
        let token = &auth_token.0;
        let allocations = AllocationStore::new(app_state, token.clone()).list().await;

        let row_template = utils::get_file_text("row_allocation.html").await;
        let rows_html = allocations.into_iter()
            .map(|a| row_template
                .replace("{auth_token}", token)
                .replace("{scheme_name}", &a.scheme_name)
                .replace("{symbol}", &a.symbol)
                .replace("{amount}", &a.amount.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        let list_template = utils::get_file_text("list_allocations.html").await;
        let list_html = list_template
            .replace("{auth_token}", token)
            .replace("{rows}", &rows_html);

        Html(list_html)
    }

    pub async fn create_or_update(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
        Form(allocation): Form<AllocationView>,
    ) -> Html<String> {
        let token = &auth_token.0;
        let _created = AllocationStore::new(app_state, token.clone()).create_or_update(allocation.clone()).await;

        let row_template = utils::get_file_text("row_allocation.html").await;
        let row_html = row_template
            .replace("{auth_token}", token)
            .replace("{scheme_name}", &allocation.scheme_name)
            .replace("{symbol}", &allocation.symbol)
            .replace("{amount}", &allocation.amount.to_string());

        Html(row_html)
    }

    pub async fn remove(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
        Path(form): Path<RemoveRequest>,
    ) -> Html<String> {
        match AllocationStore::new(app_state, auth_token.0).remove(form.scheme_name.clone(), form.symbol.clone()).await {
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
