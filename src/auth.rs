use std::sync::Arc;
use axum::extract::{OriginalUri, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use crate::app_state::AppState;

#[derive(Clone)]
pub struct AuthToken(pub String);

pub async fn token_middleware(
    State(state): State<Arc<AppState>>,
    OriginalUri(original_uri): OriginalUri,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = original_uri.path();
    let auth_token = path
        .strip_prefix("/token/")
        .and_then(|rest| rest.split('/').next())
        .filter(|t| !t.is_empty())
        .ok_or(StatusCode::BAD_REQUEST)?;

    if !auth_token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !state.rate_limiter.check(auth_token).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    request.extensions_mut().insert(AuthToken(auth_token.to_string()));

    Ok(next.run(request).await)
}
