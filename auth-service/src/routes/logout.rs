use axum::http;
use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use crate::domain::errors::AuthAPIError;
use crate::utils::auth::validate_token;
use crate::utils::constants::JWT_COOKIE_NAME;

pub async fn logout(jar: CookieJar) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let Some(cookie) = jar.get(JWT_COOKIE_NAME) else {
        return (jar, Err(AuthAPIError::MissingToken))
    };

    let cookie = cookie.to_owned();
    let token = cookie.value().to_owned();

    if let Err(_) = validate_token(&token).await {
        return (jar, Err(AuthAPIError::InvalidToken))
    }

    let jar = jar.remove(cookie);
    (jar, Ok(http::StatusCode::OK))
}
