use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;

struct WindowState {
    count: u32,
    window_start: Instant,
}

struct RateLimiterState {
    windows: HashMap<String, WindowState>,
    last_cleanup: Instant,
}

#[derive(Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimiterState>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimiterState {
                windows: HashMap::new(),
                last_cleanup: Instant::now(),
            })),
        }
    }

    pub async fn check(&self, auth_token: &str) -> bool {
        let mut state = self.state.lock().await;
        let now = Instant::now();

        if now.duration_since(state.last_cleanup).as_secs() >= 60 {
            state.windows.retain(|_, w| now.duration_since(w.window_start).as_secs() < 2);
            state.last_cleanup = now;
        }

        let window = state.windows.entry(auth_token.to_string()).or_insert_with(|| WindowState {
            count: 0,
            window_start: now,
        });

        if now.duration_since(window.window_start).as_secs() >= 1 {
            window.count = 1;
            window.window_start = now;
            true
        } else if window.count < 20 {
            window.count += 1;
            true
        } else {
            false
        }
    }
}
