use axum::http;
use axum::response::IntoResponse;

pub async fn logout() -> impl IntoResponse {
    http::StatusCode::OK.into_response()
}
