use crate::domain::data_stores::UserStoreError;
use crate::domain::errors::AuthAPIError;
use crate::domain::user::User;
use crate::domain::{Email, Password};
use crate::{AppState, AppUserStore};
use axum::{extract::State, http, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn signup<T: AppUserStore>(
    State(state): State<AppState<T>>,
    Json(params): Json<SignUpParams>,
) -> impl IntoResponse {
    let user: User;

    match params.to_user() {
        Some(u) => user = u,
        None => {
            return AuthAPIError::InvalidCredentials.into_response()
        }
    }


    let mut user_store = state.user_store.write().await;

    match user_store.add_user(user).await {
        Err(UserStoreError::UserAlreadyExists) => AuthAPIError::UserAlreadyExists.into_response(),
        Err(_) => AuthAPIError::UnexpectedError.into_response(),
        Ok(_) => (
            http::StatusCode::CREATED,
            Json(SignUpResponse::new("User created successfully!")),
        )
            .into_response(),
    }
}

#[derive(Deserialize)]
pub struct SignUpParams {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}

impl SignUpParams {
    fn to_user(&self) -> Option<User> {
        match (Email::parse(&self.email), Password::parse(&self.password)) {
            (Ok(email), Ok(password)) => {
                Some(User {
                    email,
                    password,
                    requires_2fa: self.requires_2fa,
                })
            },
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignUpResponse {
    pub message: String,
}

impl SignUpResponse {
    fn new(message: &str) -> SignUpResponse {
        SignUpResponse {
            message: message.to_string(),
        }
    }
}
