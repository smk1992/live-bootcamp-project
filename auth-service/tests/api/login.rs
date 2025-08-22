use crate::helpers::TestApp;

#[tokio::test]
async fn login_is_successful() {
    let app = TestApp::new().await;
    let response = app.post_login().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn logout_is_successful() {
    let app = TestApp::new().await;
    let response = app.post_logout().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_2fa_is_successful() {
    let app = TestApp::new().await;
    let response = app.post_verify_2fa().await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn verify_token_is_successful() {
    let app = TestApp::new().await;
    let response = app.post_verify_token().await;

    assert_eq!(response.status().as_u16(), 200);
}
