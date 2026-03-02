mod methods;
mod rest;
mod htmx;

use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::app_state::AppState;
use crate::routers::TRouter;

pub enum Api {
    REST,
    HTMX,
}

impl Api {
    pub fn get_router(&self, state: Arc<AppState>) -> Router {
        let router =
            self.with_info(
                Router::new()
            );

        router.with_state(state.clone())
    }

    fn with_info(&self, router: TRouter) -> TRouter {
        match self {
            Api::REST => router.route("/", get(rest::Methods::info)),
            Api::HTMX => router.route("/", get(htmx::Methods::info)),
        }
    }
}
