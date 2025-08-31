use auth_service::utils::constants::JWT_COOKIE_NAME;
use crate::helpers::TestApp;

#[tokio::test]
async fn verify_token_is_malformed() {
    let app = TestApp::new().await;

    let response = app.post_verify_token(&serde_json::json!({})).await;

    assert_eq!(response.status().as_u16(), 422);
}

#[tokio::test]
async fn verify_token_is_invalid() {
    let app = TestApp::new().await;

    // verify token
    let response = app.post_verify_token(&serde_json::json!({
        "token": "invalid",
    })).await;

    assert_eq!(response.status().as_u16(), 401);
}

#[tokio::test]
async fn verify_token_is_valid() {
    let app = TestApp::new().await;

    let email = "user@example.com";
    let password = "password";

    // sign up
    let response = app.post_signup(&serde_json::json!({
        "email": email,
        "password": password,
        "requires2FA": false
    })).await;
    assert_eq!(response.status().as_u16(), 201);

    // login
    let response = app.post_login(&serde_json::json!({
        "email": email,
        "password": password,
    })).await;
    assert_eq!(response.status().as_u16(), 200);

    let token = response.cookies().into_iter().find(|cookie| cookie.name() == JWT_COOKIE_NAME).unwrap();

    // verify token
    let response = app.post_verify_token(&serde_json::json!({
        "token": token.value(),
    })).await;

    assert_eq!(response.status().as_u16(), 200);
}
