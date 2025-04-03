use axum::response::Html;
use axum::Router;
use axum::routing::get;
use tower_http::services::ServeDir;
use crate::routers::utils;

pub fn get_router() -> Router {
    let assets = ServeDir::new("assets");

    Router::new()
        .route("/", get(serve_html_file))
        .nest_service("/assets", assets)
}

async fn serve_html_file() -> Html<String> {
    let html_content = utils::get_file_text("index.html").await;

    Html(html_content)
}
