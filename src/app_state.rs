use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::storage::MultiStorage;
use crate::rate_limiter::RateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<Mutex<MultiStorage>>,
    pub rate_limiter: RateLimiter,
    pub price_status: Arc<Mutex<PriceStatus>>,
}

pub struct PriceStatus {
    pub last_result: Option<String>,
    pub last_updated: Option<String>,
    pub tokens_updated: u32,
}

impl Default for PriceStatus {
    fn default() -> Self {
        Self {
            last_result: None,
            last_updated: None,
            tokens_updated: 0,
        }
    }
}

impl AppState {
    pub fn new(storage: Arc<Mutex<MultiStorage>>, rate_limiter: RateLimiter) -> Self {
        Self {
            storage,
            rate_limiter,
            price_status: Arc::new(Mutex::new(PriceStatus::default())),
        }
    }
}
