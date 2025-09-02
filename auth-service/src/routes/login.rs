use crate::domain::data_stores::{LoginAttemptId, TwoFACode, UserStoreError};
use crate::domain::errors::AuthAPIError;
use crate::domain::user::User;
use crate::domain::{Email, Password};
use crate::routes::login::LoginResponse::{RegularAuth, TwoFactorAuth};
use crate::utils::auth::{generate_auth_cookie, GenerateTokenError};
use crate::{AppState, EmailClientType, TwoFACodeStoreType};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::{http, Json};
use axum_extra::extract::CookieJar;
use log::log;
use serde::{Deserialize, Serialize};

pub async fn login(
    State(app_state): State<AppState>,
    jar: CookieJar,
    Json(params): Json<LoginParams>,
) -> (CookieJar, Result<impl IntoResponse, AuthAPIError>) {
    let Ok(email) = Email::parse(&params.email) else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };
    let Ok(password) = Password::parse(&params.password) else {
        return (jar, Err(AuthAPIError::InvalidCredentials));
    };

    // validate credentials
    let get_user = app_state
        .user_store
        .read()
        .await
        .validate_user(&email, &password)
        .await;

    match get_user {
        Err(UserStoreError::IncorrectCredentials | UserStoreError::UserNotFound) => {
            (jar, Err(AuthAPIError::IncorrectCredentials))
        }
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
        Ok(user) => {
            if user.requires_2fa {
                handle_2fa(
                    app_state.two_fa_code_store,
                    app_state.email_client,
                    jar,
                    user,
                )
                .await
            } else {
                handle_no_2fa(jar, &user.email).await
            }
        }
    }
}

async fn handle_2fa(
    two_fa_code_store: TwoFACodeStoreType,
    email_client: EmailClientType,
    jar: CookieJar,
    user: User,
) -> (
    CookieJar,
    Result<(http::StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    let (login_attempt_id, two_fa_code) = (LoginAttemptId::default(), TwoFACode::default());

    if let Err(_) = send_2fa_email(email_client, &user.email, &two_fa_code).await {
        return (jar, Err(AuthAPIError::UnexpectedError));
    }

    match two_fa_code_store
        .write()
        .await
        .add_code(
            user.email.clone(),
            login_attempt_id.clone(),
            two_fa_code.clone(),
        )
        .await
    {
        Ok(_) => (
            jar,
            Ok((
                http::StatusCode::PARTIAL_CONTENT,
                Json(TwoFactorAuth(TwoFactorAuthResponse {
                    message: "2FA required".to_string(),
                    login_attempt_id: login_attempt_id.as_ref().to_string(),
                })),
            )),
        ),
        Err(_) => (jar, Err(AuthAPIError::UnexpectedError)),
    }
}

async fn send_2fa_email(client: EmailClientType, email: &Email, code: &TwoFACode) -> Result<(), String> {
    let subject = "Login 2FA Code";
    let body = format!("The 2FA code is : {}", code.as_ref());

    client.send_email(email, subject, &body).await
}

async fn handle_no_2fa(
    jar: CookieJar,
    email: &Email,
) -> (
    CookieJar,
    Result<(http::StatusCode, Json<LoginResponse>), AuthAPIError>,
) {
    // generate cookie
    let auth_cookie = generate_auth_cookie(&email).map_err(|err| {
        match err {
            GenerateTokenError::UnexpectedError => AuthAPIError::UnexpectedError,
            GenerateTokenError::TokenError(_) => {
                // TODO: generate log entry for token error
                AuthAPIError::UnexpectedError
            }
        }
    });

    match auth_cookie {
        Ok(cookie) => (
            jar.add(cookie),
            Ok((http::StatusCode::OK, Json(RegularAuth))),
        ),
        Err(err) => (jar, Err(err)),
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum LoginResponse {
    RegularAuth,
    TwoFactorAuth(TwoFactorAuthResponse),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TwoFactorAuthResponse {
    pub message: String,
    #[serde(rename = "loginAttemptId")]
    pub login_attempt_id: String,
}

#[derive(Deserialize)]
pub struct LoginParams {
    pub email: String,
    pub password: String,
}
