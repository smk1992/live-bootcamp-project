use crate::domain::user::User;
use crate::services::hashmap_user_store::UserStoreError;
use crate::AppState;
use axum::{extract::State, http, response::IntoResponse, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};

#[debug_handler]
pub async fn signup(
    State(state): State<AppState>,
    Json(params): Json<SignUpParams>,
) -> impl IntoResponse {
    let user: User = params.into();

    let mut user_store = state.user_store.write().await;

    match user_store.add_user(user).await {
        Err(UserStoreError::UserAlreadyExists) => (
            http::StatusCode::UNPROCESSABLE_ENTITY,
            Json(SignUpResponse::new("User Already Exists")),
        ),
        Err(_) => (
            http::StatusCode::BAD_REQUEST,
            Json(SignUpResponse::new("Something went wrong")),
        ),
        Ok(_) => (
            http::StatusCode::CREATED,
            Json(SignUpResponse::new("User created successfully!")),
        ),
    }
}

#[derive(Deserialize)]
pub struct SignUpParams {
    pub email: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
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

impl From<SignUpParams> for User {
    fn from(params: SignUpParams) -> User {
        User::new(params.email, params.password, params.requires_2fa)
    }
}
