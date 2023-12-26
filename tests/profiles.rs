#![allow(clippy::bool_assert_comparison)]

use http::{header, StatusCode};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::common::{new_test_server, register_user, TEST_EMAIL, TEST_PASSWORD, TEST_USERNAME};

#[path = "common.rs"]
mod common;

#[sqlx::test]
async fn test_get_profile_no_auth(pool: Pool<Postgres>) {
    let server = new_test_server(pool);

    let _ = register_user(&server, TEST_USERNAME, TEST_EMAIL, TEST_PASSWORD).await;

    let response = server.get(&format!("/api/profiles/{TEST_USERNAME}")).await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    let body: Value = response.json();
    assert_eq!(body["profile"]["username"], TEST_USERNAME);
    assert_eq!(body["profile"]["following"].as_bool().unwrap(), false);
}

#[sqlx::test]
async fn test_get_profile_with_auth(pool: Pool<Postgres>) {
    let server = new_test_server(pool);

    let response = register_user(&server, TEST_USERNAME, TEST_EMAIL, TEST_PASSWORD).await;
    let body: Value = response.json();
    let token = body["user"]["token"].as_str().unwrap();

    let _ = register_user(&server, "followed", "followed@example.com", TEST_PASSWORD).await;

    let _ = server
        .post("/api/profiles/followed/follow")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .await;

    let response = server
        .get("/api/profiles/followed")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    let body: Value = response.json();
    assert_eq!(body["profile"]["username"], "followed");
    assert_eq!(body["profile"]["following"].as_bool().unwrap(), true);
}

#[sqlx::test]
async fn test_follow_user(pool: Pool<Postgres>) {
    let server = new_test_server(pool.clone());

    let response = register_user(&server, TEST_USERNAME, TEST_EMAIL, TEST_PASSWORD).await;
    let body: Value = response.json();
    let token = body["user"]["token"].as_str().unwrap();

    let _ = register_user(&server, "followed", "followed@example.com", TEST_PASSWORD).await;

    let response = server
        .post("/api/profiles/followed/follow")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    let body: Value = response.json();

    assert_eq!(body["profile"]["username"], "followed");
    assert_eq!(body["profile"]["following"].as_bool().unwrap(), true);
}

#[sqlx::test]
async fn test_unfollow_user(pool: Pool<Postgres>) {
    let server = new_test_server(pool);

    let response = register_user(&server, TEST_USERNAME, TEST_EMAIL, TEST_PASSWORD).await;
    let body: Value = response.json();
    let token = body["user"]["token"].as_str().unwrap();

    let _ = register_user(&server, "followed", "followed@example.com", TEST_PASSWORD).await;

    let _ = server
        .post("/api/profiles/followed/follow")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .await;

    let response = server
        .delete("/api/profiles/followed/follow")
        .add_header(
            header::AUTHORIZATION,
            format!("Token {token}").try_into().unwrap(),
        )
        .await;

    assert_eq!(response.status_code(), StatusCode::OK);
    assert_eq!(
        response.header(header::CONTENT_TYPE),
        mime::APPLICATION_JSON.as_ref()
    );
    let body: Value = response.json();
    assert_eq!(body["profile"]["username"], "followed");
    assert_eq!(body["profile"]["following"].as_bool().unwrap(), false);
}
