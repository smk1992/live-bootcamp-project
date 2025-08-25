extern crate core;

mod domain;
mod routes;
pub use routes::SignUpResponse;
mod services;
pub use crate::services::hashmap_user_store::HashMapUserStore;

use axum::{
    response::Html,
    routing::{get, post},
    serve::Serve,
    Router,
};
use std::{error::Error, sync::Arc};
use tokio::sync::RwLock;

use tower_http::services::ServeDir;

pub type UserStoreType = Arc<RwLock<HashMapUserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
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
    pub async fn build(app_state: AppState, address: &str) -> Result<Self, Box<dyn Error>> {
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

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>Hello, World! Done Task 1 For Rusty Bootcamp!</h1>")
}
