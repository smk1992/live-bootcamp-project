use axum::http;
use axum::response::IntoResponse;

pub async fn verify_2fa() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
