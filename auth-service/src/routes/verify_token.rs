use axum::{http, Json};
use serde::Deserialize;
use crate::domain::errors::AuthAPIError;
use crate::utils::auth::validate_token;

pub async fn verify_token(
    Json(params): Json<VerifyTokenParams>,
) -> Result<http::StatusCode, AuthAPIError> {
    validate_token(&params.token).await.map_err(|_| AuthAPIError::InvalidToken)?;

    Ok(http::StatusCode::OK)
}

#[derive(Deserialize)]
pub struct VerifyTokenParams {
    pub token: String,
}
