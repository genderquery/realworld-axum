use std::time::Duration;

use axum_test::TestServer;
use conduit::{app, jwt, AppState};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::{Pool, Postgres};

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
