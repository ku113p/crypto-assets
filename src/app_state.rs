use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::storage::MultiStorage;
use crate::rate_limiter::RateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Mutex<MultiStorage>>,
    pub rate_limiter: RateLimiter,
}

impl AppState {
    pub fn new(storage: Arc<Mutex<MultiStorage>>, rate_limiter: RateLimiter) -> Self {
        Self { storage, rate_limiter }
    }
}
