use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::storage::Storage;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Mutex<Storage>>,
}

impl AppState {
    pub fn new(storage: Arc<Mutex<Storage>>) -> Self {
        Self { storage }
    }
}
