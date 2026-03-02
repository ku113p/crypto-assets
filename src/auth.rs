use std::sync::Arc;
use axum::extract::{Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use crate::app_state::AppState;

#[derive(Clone)]
pub struct AuthToken(pub String);

pub async fn token_middleware(
    State(state): State<Arc<AppState>>,
    Path(auth_token): Path<String>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if auth_token.is_empty()
        || !auth_token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !state.rate_limiter.check(&auth_token).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    request.extensions_mut().insert(AuthToken(auth_token));

    Ok(next.run(request).await)
}
