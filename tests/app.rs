use axum::http::StatusCode;
use axum_test::TestServer;
use conduit::{app, AppState};
use sqlx::PgPool;

#[sqlx::test]
async fn test_app(pool: PgPool) {
    let state = AppState { pool };
    let server = TestServer::new(app(state)).unwrap();
    let response = server.get("/").await;
    assert_eq!(response.status_code(), StatusCode::OK);
}
