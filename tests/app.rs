use axum::http::StatusCode;
use axum_test::TestServer;
use conduit::{
    app,
    jwt::{self, Jwt},
    AppState,
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use sqlx::PgPool;

#[sqlx::test]
async fn test_app(pool: PgPool) {
    let jwt = Jwt::new(jwt::Config {
        encoding_key: EncodingKey::from_secret(b"secret"),
        decoding_key: DecodingKey::from_secret(b"secret"),
        duration_secs: 15 * 60,
    });
    let state = AppState { pool, jwt };
    let server = TestServer::new(app(state)).unwrap();
    let response = server.get("/").await;
    assert_eq!(response.status_code(), StatusCode::OK);
}
