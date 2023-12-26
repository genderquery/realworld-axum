use std::time::Duration;

use axum_test::{TestResponse, TestServer};
use conduit::{app, jwt, AppState};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde_json::json;
use sqlx::{Pool, Postgres};

pub const TEST_USERNAME: &str = "AzureDiamond";
pub const TEST_EMAIL: &str = "example@example.com";
pub const TEST_PASSWORD: &str = "hunter2";

pub fn new_test_server(pool: Pool<Postgres>) -> TestServer {
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

#[allow(dead_code)]
pub async fn register_user(
    server: &TestServer,
    username: &str,
    email: &str,
    password: &str,
) -> TestResponse {
    let payload = json!({
        "user": {
            "username": username,
            "email": email,
            "password": password
        }
    });

    server.post("/api/users").json(&payload).await
}
