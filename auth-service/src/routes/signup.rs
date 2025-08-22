use axum::{response::IntoResponse, http, Json};
use serde::Deserialize;

pub async fn signup(Json(params): Json<SignUpParams>) -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}

#[derive(Deserialize)]
pub struct SignUpParams {
    pub username: String,
    pub password: String,
    #[serde(rename = "requires2FA")]
    pub requires_2fa: bool,
}
