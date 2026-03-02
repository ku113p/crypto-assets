use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use axum::Router;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use log::{error, info};
use tokio::sync::Mutex;
use tokio::time::interval;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use crate::app_state::AppState;
use crate::models::storage::{MultiStorage, StorageOperator};
use crate::rate_limiter::RateLimiter;

mod models;
mod app_state;
mod routers;
mod utils;
mod auth;
mod rate_limiter;
mod price_worker;

#[tokio::main]
async fn main() {
    env_logger::init();

    let storage_path = std::env::var("STORAGE_PATH").unwrap_or_else(|_| "./storage.bin".to_string());
    let storage_operator = StorageOperator::new(storage_path);
    let mut multi_storage = match load_storage(&storage_operator) {
        Ok(storage) => storage,
        Err(err) => {
            error!("{err}");
            return;
        }
    };

    // Ensure workspace "0" exists
    multi_storage.get_or_create("0");

    let storage = Arc::new(Mutex::new(multi_storage));
    let rate_limiter = RateLimiter::new();
    let state = Arc::new(AppState::new(storage.clone(), rate_limiter));

    spawn_storage_saver(storage.clone(), storage_operator.clone());
    price_worker::spawn_price_worker(storage.clone());

    if let Err(err) = run_server(state).await {
        error!("Server error: {err:?}");
    }
}

fn load_storage(storage_operator: &StorageOperator) -> Result<MultiStorage, String> {
    match storage_operator.load() {
        Ok(Some(storage)) => {
            info!("Storage successfully loaded");
            Ok(storage)
        },
        Ok(None) => {
            info!("Storage not exists. Default will be created.");
            Ok(MultiStorage::default())
        }
        Err(err) => Err(format!("Failed to load storage: {err:?}"))
    }
}

fn spawn_storage_saver(storage: Arc<Mutex<MultiStorage>>, storage_operator: StorageOperator) {
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        interval.tick().await;
        loop {
            interval.tick().await;
            let storage_guard = storage.lock().await;
            match storage_operator.save(&storage_guard) {
                Ok(_) => info!("Storage successfully saved."),
                Err(err) => error!("Failed to save storage: {err:?}"),
            }
        }
    });
}

async fn run_server(state: Arc<AppState>) -> Result<(), Box<dyn Error>> {
    let token_routes = Router::new()
        .route("/", get(routers::index::dashboard))
        .route("/dashboard", get(routers::index::dashboard))
        .nest("/api", routers::get_router(state.clone()))
        .layer(from_fn_with_state(state.clone(), auth::token_middleware));

    let router = Router::new()
        .route("/", get(routers::index::landing_page))
        .route("/ping", get(utils::ping))
        .nest("/token/{auth_token}", token_routes)
        .nest_service("/assets", ServeDir::new("assets"))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3999").await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
