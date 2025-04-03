use axum::response::Html;
use axum::Router;
use axum::routing::get;
use crate::routers::utils;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(serve_html_file))
}

async fn serve_html_file() -> Html<String> {
    let html_content = utils::get_file_text("index.html").await;

    Html(html_content)
}
