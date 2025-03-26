mod methods;
mod rest;
mod htmx;

use std::sync::Arc;
use axum::Router;
use axum::routing::{delete, get, post};
use crate::app_state::AppState;
use crate::routers::TRouter;
use crate::utils as base_utils;

pub enum Api {
    REST,
    HTMX,
}

impl Api {
    pub fn get_router(&self, state: Arc<AppState>) -> Router {
        let router =
            self.with_list(self.with_create(self.with_remove(
                Router::new().route("/ping", get(base_utils::ping))
            )));

        router.with_state(state.clone())
    }

    fn with_list(&self, router: TRouter) -> TRouter {
        match self {
            Api::REST => router.route("/", get(rest::Methods::list)),
            Api::HTMX => router.route("/", get(htmx::Methods::list)),
        }
    }

    fn with_create(&self, router: TRouter) -> TRouter {
        match self {
            Api::REST => router.route("/", post(rest::Methods::create_or_update)),
            Api::HTMX => router.route("/", post(htmx::Methods::create_or_update)),
        }
    }

    fn with_remove(&self, router: TRouter) -> TRouter {
        match self {
            Api::REST => router.route("/{scheme_name}/{symbol}", delete(rest::Methods::remove)),
            Api::HTMX => router.route("/{scheme_name}/{symbol}", delete(htmx::Methods::remove))
        }
    }
}
