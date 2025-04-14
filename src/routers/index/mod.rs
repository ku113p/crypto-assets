use axum::Router;
use tower_http::services::ServeDir;

pub fn get_router() -> Router {
    let serve_dir = ServeDir::new("assets");

    Router::new()
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
}
