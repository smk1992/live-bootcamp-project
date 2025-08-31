extern crate core;

mod domain;
mod routes;
pub use routes::SignUpResponse;
mod services;
pub mod utils;

pub use crate::services::hashmap_user_store::HashMapUserStore;

use crate::utils::auth::GenerateTokenError;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{
    response::Html,
    routing::{get, post},
    serve::Serve,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;

use crate::domain::data_stores::UserStore;
use crate::domain::errors::AuthAPIError;
use tower_http::services::ServeDir;

pub trait AppUserStore: UserStore + Clone + Send + Sync {}
impl<T: UserStore + Clone + Send + Sync> AppUserStore for T {}

#[derive(Clone)]
pub struct AppState<T: AppUserStore> {
    pub user_store: Arc<RwLock<T>>,
}

impl<T: AppUserStore> AppState<T> {
    pub fn new(user_store: T) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
        }
    }
}

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    pub async fn build<T: AppUserStore + 'static>(
        app_state: AppState<T>,
        address: &str,
    ) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point!
        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .route("/hello", get(hello_handler))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        Ok(Application {
            server,
            address: address.to_string(),
        })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AuthAPIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthAPIError::UserAlreadyExists => (StatusCode::CONFLICT, "User already exists"),
            AuthAPIError::InvalidCredentials => (StatusCode::BAD_REQUEST, "Invalid credentials"),
            AuthAPIError::IncorrectCredentials => {
                (StatusCode::UNAUTHORIZED, "Incorrect credentials")
            }
            AuthAPIError::UnexpectedError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Unexpected error")
            }
        };
        let body = Json(ErrorResponse {
            error: error_message.to_string(),
        });

        (status, body).into_response()
    }
}

impl IntoResponse for GenerateTokenError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            GenerateTokenError::TokenError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
            GenerateTokenError::UnexpectedError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World! Done Task 1 For Rusty Bootcamp!</h1>")
}
