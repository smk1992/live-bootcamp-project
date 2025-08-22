use axum::http;
use axum::response::IntoResponse;

pub async fn signup() -> impl IntoResponse {
    println!("It comes here!");
    http::StatusCode::OK.into_response()
}
