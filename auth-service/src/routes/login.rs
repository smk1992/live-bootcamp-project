use crate::domain::data_stores::UserStoreError;
use crate::domain::errors::AuthAPIError;
use crate::domain::{Email, Password};
use crate::utils::auth::{generate_auth_cookie, GenerateTokenError};
use crate::{AppState, AppUserStore};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{http, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;

pub async fn login<T: AppUserStore>(
    State(app_state): State<AppState<T>>,
    jar: CookieJar,
    Json(params): Json<LoginParams>,
) -> Result<(CookieJar, http::StatusCode), impl IntoResponse> {
    let Ok(email) = Email::parse(&params.email) else {
        return Err(AuthAPIError::InvalidCredentials);
    };
    let Ok(password) = Password::parse(&params.password) else {
        return Err(AuthAPIError::InvalidCredentials);
    };

    // validate credentials
    let result = app_state
        .user_store
        .read()
        .await
        .validate_user(&email, &password)
        .await;
    result.map_err(|err| match err {
        UserStoreError::IncorrectCredentials | UserStoreError::UserNotFound => {
            AuthAPIError::IncorrectCredentials
        }
        _ => AuthAPIError::UnexpectedError,
    })?;

    // generate cookie
    let auth_cookie = generate_auth_cookie(&email).map_err(|err| {
        match err {
            GenerateTokenError::UnexpectedError => AuthAPIError::UnexpectedError,
            GenerateTokenError::TokenError(err) => {
                // TODO: generate log entry for token error
                AuthAPIError::UnexpectedError
            }
        }
    })?;

    Ok((jar.add(auth_cookie), http::StatusCode::OK))
}

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}
