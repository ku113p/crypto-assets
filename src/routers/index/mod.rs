use axum::Extension;
use axum::response::Html;
use crate::auth::AuthToken;
use crate::routers::utils;

pub async fn dashboard(Extension(auth_token): Extension<AuthToken>) -> Html<String> {
    let token = auth_token.0;
    let template = utils::get_file_text("dashboard.html").await;
    Html(template.replace("{auth_token}", &token))
}

pub async fn landing_page() -> Html<String> {
    let template = utils::get_file_text("landing.html").await;
    Html(template)
}
