use std::sync::Arc;
use axum::extract::{Request, State};
use axum::http::{StatusCode, Uri};
use axum::middleware::Next;
use axum::response::Response;
use crate::app_state::AppState;

#[derive(Clone)]
pub struct AuthToken(pub String);

pub async fn token_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();
    let query: Option<String> = request.uri().query().map(|q| q.to_string());

    if let Some(rest) = path.strip_prefix("/token/") {
        let (auth_token, new_path) = match rest.find('/') {
            Some(pos) => (&rest[..pos], &rest[pos..]),
            None => (rest, "/dashboard"),
        };

        if !auth_token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
            return Err(StatusCode::BAD_REQUEST);
        }

        if !state.rate_limiter.check(auth_token).await {
            return Err(StatusCode::TOO_MANY_REQUESTS);
        }

        request.extensions_mut().insert(AuthToken(auth_token.to_string()));

        let new_uri = match query {
            Some(q) => Uri::builder()
                .path_and_query(format!("{new_path}?{q}"))
                .build()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
            None => Uri::builder()
                .path_and_query(new_path)
                .build()
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        };
        *request.uri_mut() = new_uri;
    }

    Ok(next.run(request).await)
}
