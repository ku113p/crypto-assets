use axum::response::Html;
use axum::Router;
use axum::routing::get;
use tokio::fs;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(serve_html_file))
        .route("/message", get(serve_message))
}

async fn serve_html_file() -> Html<String> {
    let html_content = fs::read_to_string("templates/index.html")
        .await
        .unwrap_or_else(|_| "<h1>Error: HTML file not found</h1>".to_string());

    Html(html_content)
}

async fn serve_message() -> Html<&'static str> {
    Html("<p><strong>HTMX Loaded this message dynamically! 🚀</strong></p>")
}
