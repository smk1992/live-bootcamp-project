use crate::domain::data_stores::{LoginAttemptId, TwoFACode, TwoFACodeStore, TwoFACodeStoreError};
use crate::domain::errors::AuthAPIError;
use crate::domain::errors::AuthAPIError::UnexpectedError;
use crate::domain::Email;
use crate::utils::auth::generate_auth_cookie;
use crate::AppState;
use axum::extract::State;
use axum::{http, Json};
use axum_extra::extract::CookieJar;
use serde::Deserialize;
use tokio::sync::RwLockWriteGuard;

pub async fn verify_2fa(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(params): Json<Verify2FAParams>,
) -> (CookieJar, Result<http::StatusCode, AuthAPIError>) {
    let (Ok(email), Ok(login_attempt_id), Ok(code)) = (
        params.parse_email(),
        params.parse_login_attempt(),
        params.parse_two_fa_code(),
    ) else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    let store = app_state.two_fa_code_store.write().await;

    match store.get_code(&email).await {
        Err(TwoFACodeStoreError::UnexpectedError) => (jar, Err(AuthAPIError::UnexpectedError)),
        Err(TwoFACodeStoreError::LoginAttemptIdNotFound) => {
            (jar, Err(AuthAPIError::IncorrectCredentials))
        }
        Ok((stored_login_attempt_id, stored_two_fa_code)) => {
            if login_attempt_id == stored_login_attempt_id && stored_two_fa_code == code {
                setup_auth(email, jar, store).await
            } else {
                (jar, Err(AuthAPIError::IncorrectCredentials))
            }
        }
    }
}

async fn setup_auth(
    email: Email,
    jar: CookieJar,
    mut two_fa_code_store: RwLockWriteGuard<'_, dyn TwoFACodeStore + Send + Sync>,
) -> (CookieJar, Result<http::StatusCode, AuthAPIError>) {
    let Ok(auth_cookie) = generate_auth_cookie(&email) else {
        return (jar, Err(UnexpectedError));
    };

    if two_fa_code_store.remove_code(&email).await.is_err() {
        return (jar, Err(UnexpectedError));
    }

    (jar.add(auth_cookie), Ok(http::StatusCode::OK))
}

#[derive(Deserialize)]
pub struct Verify2FAParams {
    pub email: String,

    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,

    #[serde(rename = "2FACode")]
    pub code: String,
}

impl Verify2FAParams {
    fn parse_email(&self) -> Result<Email, ()> {
        Email::parse(&self.email)
    }

    fn parse_login_attempt(&self) -> Result<LoginAttemptId, String> {
        LoginAttemptId::parse(self.login_attempt_id.clone())
    }

    fn parse_two_fa_code(&self) -> Result<TwoFACode, String> {
        TwoFACode::parse(self.code.clone())
    }
}
