use axum::http::StatusCode;
use axum_test::TestServer;
use conduit::app;

#[tokio::test]
async fn test_app() {
    let server = TestServer::new(app()).unwrap();
    let response = server.get("/").await;
    assert_eq!(response.status_code(), StatusCode::OK);
}
