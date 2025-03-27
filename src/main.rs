use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use axum::Router;
use axum::routing::get;
use log::{error, info};
use tokio::sync::Mutex;
use tokio::time::interval;
use tower_http::trace::TraceLayer;
use crate::app_state::AppState;
use crate::models::storage::{Storage, StorageOperator};

mod models;
mod app_state;
mod routers;
mod utils;

#[tokio::main]
async fn main() {
    env_logger::init();

    let storage_operator = StorageOperator::new("./storage.bin".to_string());
    let storage = Arc::new(Mutex::new(match load_storage(&storage_operator) {
        Ok(storage) => storage,
        Err(err) => {
            error!("{err}");
            return;
        }
    }));
    let state = Arc::new(AppState::new(storage.clone()));

    spawn_storage_saver(storage.clone(), storage_operator.clone());

    if let Err(err) = run_server(state).await {
        error!("Server error: {err:?}");
    }
}

fn load_storage(storage_operator: &StorageOperator) -> Result<Storage, String> {
    match storage_operator.load() {
        Ok(Some(storage)) => {
            info!("Storage successfully loaded");
            Ok(storage)
        },
        Ok(None) => {
            info!("Storage not exists. Default will be created.");
            Ok(Storage::default())
        }
        Err(err) => Err(format!("Failed to load storage: {err:?}"))
    }
}

fn spawn_storage_saver(storage: Arc<Mutex<Storage>>, storage_operator: StorageOperator) {
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
    let router = Router::new()
        .merge(routers::index::get_router())
        .route("/ping", get(utils::ping))
        .nest("/api", routers::get_router(state))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3999").await?;
    axum::serve(listener, router.into_make_service()).await?;

    Ok(())
}
