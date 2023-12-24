use std::time::Duration;

use axum_test::TestServer;
use conduit::{app, jwt, AppState};
use http::{header, StatusCode};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

const TEST_USERNAME: &str = "AzureDiamond";
const TEST_EMAIL: &str = "example@example.com";
const TEST_PASSWORD: &str = "hunter2";

async fn new_test_server(pool: Pool<Postgres>) -> TestServer {
    let app_state = AppState {
        jwt: jwt::Config {
            expiration: Duration::from_secs(3600),
            encoding_key: EncodingKey::from_secret(b"secret"),
            decoding_key: DecodingKey::from_secret(b"secret"),
        },
        db: pool,
    };
    TestServer::new(app(app_state)).unwrap()
}

#[sqlx::test]
async fn test_registration(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "email": TEST_EMAIL,
            "password": TEST_PASSWORD
        }
    });

    let response = server.post("/api/users").json(&payload).await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::CREATED);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["user"]["username"], payload["user"]["username"]);
    assert_eq!(body["user"]["email"], payload["user"]["email"]);
    assert!(body["user"]["token"].is_string());
}

#[sqlx::test]
async fn test_registration_validation(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": "",
            "email": "",
            "password": ""
        }
    });

    let response = server.post("/api/users").json(&payload).await;
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

#[sqlx::test]
async fn test_registration_existing_credentials(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "email": TEST_EMAIL,
            "password": TEST_PASSWORD
        }
    });

    let _ = server.post("/api/users").json(&payload).await;

    let response = server.post("/api/users").json(&payload).await;

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[sqlx::test]
async fn test_login(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "email": TEST_EMAIL,
            "password": TEST_PASSWORD
        }
    });

    let _ = server.post("/api/users").json(&payload).await;

    let login_payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "password": TEST_PASSWORD
        }
    });

    let response = server.post("/api/users/login").json(&login_payload).await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["user"]["username"], payload["user"]["username"]);
    assert_eq!(body["user"]["email"], payload["user"]["email"]);
    assert!(body["user"]["token"].is_string());
}

#[sqlx::test]
async fn test_login_validation(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": "",
            "password": ""
        }
    });

    let response = server.post("/api/users/login").json(&payload).await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["errors"]["username"][0], "can't be blank");
    assert_eq!(body["errors"]["password"][0], "can't be blank");
}

#[sqlx::test]
async fn test_login_invalid_credentials(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let paylaod = json!({
        "user": {
            "username": TEST_USERNAME,
            "email": TEST_EMAIL,
            "password": TEST_PASSWORD
        }
    });

    let _ = server.post("/api/users").json(&paylaod).await;

    let payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "password": "incorrect"
        }
    });

    let response = server.post("/api/users/login").json(&payload).await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_get_current_user(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "email": TEST_EMAIL,
            "password": TEST_PASSWORD
        }
    });

    let response = server.post("/api/users").json(&payload).await;
    let body: Value = response.json();
    let token = body["user"]["token"].as_str().unwrap();

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
    assert_eq!(body["user"]["username"], payload["user"]["username"]);
    assert_eq!(body["user"]["email"], payload["user"]["email"]);
    assert!(body["user"]["token"].is_string());
}

#[sqlx::test]
async fn test_get_current_user_no_token(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;
    let response = server.get("/api/user").await;
    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_update_user(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "email": TEST_EMAIL,
            "password": TEST_PASSWORD
        }
    });

    let response = server.post("/api/users").json(&payload).await;
    let body: Value = response.json();
    let token = body["user"]["token"].as_str().unwrap();

    let payload = json!({
        "user": {
            "username": "newusername",
            "email": "newemail@example.com",
            "password": "newpassword"
        }
    });

    let response = server
        .put("/api/user")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .json(&payload)
        .await;
    let body: Value = response.json();

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    assert_eq!(body["user"]["username"], payload["user"]["username"]);
    assert_eq!(body["user"]["email"], payload["user"]["email"]);
    assert!(body["user"]["token"].is_string());
}

#[sqlx::test]
async fn test_update_user_no_token(pool: Pool<Postgres>) {
    let server = new_test_server(pool).await;

    let payload = json!({
        "user": {
            "username": TEST_USERNAME,
            "email": TEST_EMAIL,
            "password": TEST_PASSWORD
        }
    });

    let response = server.put("/api/user").json(&payload).await;

    assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
}
