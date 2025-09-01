extern crate core;

mod domain;
mod routes;
pub use routes::SignUpResponse;
mod services;
pub mod utils;

pub use crate::services::hashmap_user_store::HashMapUserStore;
pub use crate::services::hashset_banned_token_store::HashSetBannedTokenStore;

use crate::utils::auth::GenerateTokenError;

use crate::domain::data_stores::{BannedTokenStore, UserStore};
use crate::domain::errors::AuthAPIError;
use axum::http::{Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{
    routing::{post},
    serve::Serve,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;
use tower_http::{cors::CorsLayer, services::ServeDir};

pub trait AppUserStore: UserStore + Clone + Send + Sync {}
impl<T: UserStore + Clone + Send + Sync> AppUserStore for T {}

pub type BannedStoreType = Arc<RwLock<dyn BannedTokenStore + Send + Sync>>;

#[derive(Clone)]
pub struct AppState<T: AppUserStore> {
    pub user_store: Arc<RwLock<T>>,
    pub banned_token_store: BannedStoreType
}

impl<T: AppUserStore> AppState<T> {
    pub fn new(user_store: T, banned_token_store: BannedStoreType) -> Self {
        Self {
            user_store: Arc::new(RwLock::new(user_store)),
            banned_token_store,
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
        let allowed_origins = [
            "http://localhost:8000".parse()?,
            "http://138.197.170.32:8000".parse()?,
        ];

        let cors = CorsLayer::new()
            // Allow GET and POST requests
            .allow_methods([Method::GET, Method::POST])
            // Allow cookies to be included in requests
            .allow_credentials(true)
            .allow_origin(allowed_origins);

        let router = Router::new()
            .nest_service("/", ServeDir::new("assets"))
            .route("/signup", post(routes::signup))
            .route("/login", post(routes::login))
            .route("/logout", post(routes::logout))
            .route("/verify-2fa", post(routes::verify_2fa))
            .route("/verify-token", post(routes::verify_token))
            .with_state(app_state)
            .layer(cors);

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
            AuthAPIError::MissingToken => (StatusCode::BAD_REQUEST, "Missing token"),
            AuthAPIError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid token"),
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
