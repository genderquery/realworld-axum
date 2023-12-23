use std::time::Duration;

use axum_test::TestServer;
use conduit::{app, jwt, AppState, MockDb};
use http::{header, StatusCode};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde_json::{json, Value};

const TEST_USERNAME: &str = "AzureDiamond";
const TEST_EMAIL: &str = "example@example.com";
const TEST_PASSWORD: &str = "hunter2";

fn new_test_server() -> TestServer {
    let app_state = AppState {
        jwt: jwt::Config {
            expiration: Duration::from_secs(3600),
            encoding_key: EncodingKey::from_secret(b"secret"),
            decoding_key: DecodingKey::from_secret(b"secret"),
        },
        db: MockDb::default(),
    };
    TestServer::new(app(app_state)).unwrap()
}

#[tokio::test]
async fn test_registration() {
    let server = new_test_server();

    let user = json!({
        "username": TEST_USERNAME,
        "email": TEST_EMAIL,
        "password": TEST_PASSWORD
    });

    let response = server.post("/api/users").json(&user).await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::CREATED);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["username"], user["username"]);
    assert_eq!(body["email"], user["email"]);
    assert!(body["token"].is_string());
}

#[tokio::test]
async fn test_registration_validation() {
    let server = new_test_server();

    let user = json!({
        "username": "",
        "email": "",
        "password": ""
    });

    let response = server.post("/api/users").json(&user).await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["errors"]["username"][0], "can't be blank");
    assert_eq!(body["errors"]["email"][0], "can't be blank");
    assert_eq!(body["errors"]["password"][0], "can't be blank");
}

#[tokio::test]
async fn test_registration_existing_credentials() {
    let server = new_test_server();

    let user = json!({
        "username": TEST_USERNAME,
        "email": TEST_EMAIL,
        "password": TEST_PASSWORD
    });

    let _ = server.post("/api/users").json(&user).await;

    let response = server.post("/api/users").json(&user).await;

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_login() {
    let server = new_test_server();

    let user = json!({
        "username": TEST_USERNAME,
        "email": TEST_EMAIL,
        "password": TEST_PASSWORD
    });

    let _ = server.post("/api/users").json(&user).await;

    let login_user = json!({
        "username": TEST_USERNAME,
        "password": TEST_PASSWORD
    });

    let response = server.post("/api/users/login").json(&login_user).await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["username"], user["username"]);
    assert_eq!(body["email"], user["email"]);
    assert!(body["token"].is_string());
}

#[tokio::test]
async fn test_login_validation() {
    let server = new_test_server();

    let user = json!({
        "username": "",
        "password": ""
    });

    let response = server.post("/api/users/login").json(&user).await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["errors"]["username"][0], "can't be blank");
    assert_eq!(body["errors"]["password"][0], "can't be blank");
}

#[tokio::test]
async fn test_login_invalid_credentials() {
    let server = new_test_server();

    let user = json!({
        "username": TEST_USERNAME,
        "email": TEST_EMAIL,
        "password": TEST_PASSWORD
    });

    let _ = server.post("/api/users").json(&user).await;

    let login_user = json!({
        "username": TEST_USERNAME,
        "password": "incorrect"
    });

    let response = server.post("/api/users/login").json(&login_user).await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_get_current_user() {
    let server = new_test_server();

    let user = json!({
        "username": TEST_USERNAME,
        "email": TEST_EMAIL,
        "password": TEST_PASSWORD
    });

    let response = server.post("/api/users").json(&user).await;
    let body: Value = response.json();
    let token = body["token"].as_str().unwrap();

    let response = server
        .get("/api/user")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["username"], user["username"]);
    assert_eq!(body["email"], user["email"]);
    assert!(body["token"].is_string());
}

#[tokio::test]
async fn test_get_current_user_no_token() {
    let server = new_test_server();
    let response = server.get("/api/user").await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_update_user() {
    let server = new_test_server();

    let user = json!({
        "username": TEST_USERNAME,
        "email": TEST_EMAIL,
        "password": TEST_PASSWORD
    });

    let response = server.post("/api/users").json(&user).await;
    let body: Value = response.json();
    let token = body["token"].as_str().unwrap();

    let update_user = json!({
        "username": "newusername",
        "email": "newemail@example.com",
        "password": "newpassword"
    });

    let response = server
        .put("/api/user")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .json(&update_user)
        .await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["username"], update_user["username"]);
    assert_eq!(body["email"], update_user["email"]);
    assert!(body["token"].is_string());
}

#[tokio::test]
async fn test_update_user_no_token() {
    let server = new_test_server();

    let update_user = json!({
        "username": TEST_USERNAME,
        "email": TEST_EMAIL,
        "password": TEST_PASSWORD
    });

    let response = server.put("/api/user").json(&update_user).await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}
