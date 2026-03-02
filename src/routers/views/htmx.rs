use std::sync::Arc;
use axum::extract::State;
use axum::Extension;
use axum::response::Html;
use crate::app_state::AppState;
use crate::auth::AuthToken;
use crate::routers::utils;
use crate::routers::views::methods::Views;

pub struct Methods;

impl Methods {
    pub async fn info(
        State(app_state): State<Arc<AppState>>,
        Extension(auth_token): Extension<AuthToken>,
    ) -> Html<String> {
        let token_infos = Views::new(app_state, auth_token.0).info().await;

        let row_template = utils::get_file_text("view_info_row.html").await;
        let rows_html = token_infos.into_iter()
            .map(|t| row_template
                .replace("{symbol}", &t.symbol)
                .replace("{amount}", &t.amount.to_string())
                .replace("{defi_amount}", &t.defi_amount.to_string())
                .replace("{defi_self_percentage}", &t.defi_self_percentage.to_string())
                .replace("{usdt_rate}", &t.usdt_rate.to_string())
                .replace("{usdt_amount}", &t.usdt_amount.to_string())
                .replace("{usdt_percentage}", &t.usdt_percentage.to_string())
                .replace("{usdt_defi_amount}", &t.usdt_defi_amount.to_string())
                .replace("{usdt_defi_percentage}", &t.usdt_defi_percentage.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        let list_template = utils::get_file_text("view_info.html").await;
        let list_html = list_template.replace("{rows}", &rows_html);

        Html(list_html)
    }
}
