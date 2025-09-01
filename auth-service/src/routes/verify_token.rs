use axum::{http, Json};
use axum::extract::State;
use serde::Deserialize;
use crate::{AppState, AppUserStore};
use crate::domain::errors::AuthAPIError;
use crate::utils::auth::validate_token;

pub async fn verify_token<T: AppUserStore>(
    State(app_state): State<AppState<T>>,
    Json(params): Json<VerifyTokenParams>,
) -> Result<http::StatusCode, AuthAPIError> {
    validate_token(&params.token).await.map_err(|_| AuthAPIError::InvalidToken)?;
    if app_state.banned_token_store.read().await.contains(&params.token).await {
        return Err(AuthAPIError::InvalidToken)
    }

    Ok(http::StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenParams {
    pub token: String,
}
