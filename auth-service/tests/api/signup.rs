use crate::helpers::{get_random_email, TestApp};
use auth_service::{ErrorResponse, SignUpResponse};

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        serde_json::json!({
            "password": "some_password",
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": random_email,
            "requires2FA": true,
        }),
        serde_json::json!({
            "email": random_email,
            "password": "",
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}

#[tokio::test]
async fn should_return_201_if_successful() {
    let app = TestApp::new().await;

    let json = serde_json::json!({
        "email": get_random_email(),
        "password": "a_password",
        "requires2FA": true
    });

    let response = app.post_signup(&json).await;
    assert_eq!(response.status().as_u16(), 201);

    let expected_response = SignUpResponse {
        message: "User created successfully!".to_owned(),
    };

    // Assert that we are getting the correct response body!
    assert_eq!(
        response
            .json::<SignUpResponse>()
            .await
            .expect("Could not deserialize response body to UserBody"),
        expected_response
    );
}

#[tokio::test]
async fn should_return_400_if_invalid_input() {
    let app = TestApp::new().await;

    let random_email = get_random_email();

    let test_cases = [
        // blank email
        serde_json::json!({
            "email": "",
            "password": "some_password",
            "requires2FA": true,
        }),
        // missing @
        serde_json::json!({
            "email": "aaaa",
            "password": "some_password",
            "requires2FA": true,
        }),
        // password too short
        serde_json::json!({
            "email": random_email,
            "password": "1234567",
            "requires2FA": true
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(&test_case).await;
        assert_eq!(
            response.status().as_u16(),
            400,
            "Failed for input: {:?}",
            test_case
        );
        assert_eq!(
            response
                .json::<ErrorResponse>()
                .await
                .expect("Could not deserialize response body to ErrorResponse")
                .error,
            "Invalid credentials".to_owned()
        );
    }
}

#[tokio::test]
async fn should_return_409_if_email_already_exists() {
    let app = TestApp::new().await;

    let json = serde_json::json!({
        "email": get_random_email(),
        "password": "a_password",
        "requires2FA": true
    });

    let response = app.post_signup(&json).await;
    assert_eq!(response.status().as_u16(), 201);

    let response = app.post_signup(&json).await;
    assert_eq!(response.status().as_u16(), 409);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "User already exists".to_owned()
    )
}
