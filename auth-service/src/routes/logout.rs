use crate::domain::errors::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;
use crate::{AppState};
use axum::extract::State;
use axum::http;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;

pub async fn logout(
    State(app_state): State<AppState>,
    jar: CookieJar,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let Some(cookie) = jar.get(JWT_COOKIE_NAME) else {
        return (jar, Err(AuthAPIError::MissingToken));
    };

    let cookie = cookie.to_owned();
    let token = cookie.value().to_owned();

    if let Err(_) = validate_token(&token).await {
        return (jar, Err(AuthAPIError::InvalidToken));
    }

    app_state.banned_token_store.write().await.add(token).await;

    let jar = jar.remove(cookie);
    (jar, Ok(http::StatusCode::OK))
}
