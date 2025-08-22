use axum::http;
use axum::response::IntoResponse;

pub async fn verify_token() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
