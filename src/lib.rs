use std::error::Error as StdError;

use axum::{
    extract::{FromRef, State},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http::StatusCode;
use jwt::{Claims, TokenError};

pub mod jwt;

pub fn app(state: AppState) -> Router {
    // build our application with a single route
    Router::new()
        .route("/", get(requires_authorization))
        .route("/token", get(get_token))
        .with_state(state)
}

async fn get_token(State(state): State<AppState>) -> Result<String, AppError> {
    let token = jwt::create_token(&state.jwt, "example", "example@example.com")?;
    Ok(token)
}

async fn requires_authorization(_: Claims) -> Result<String, AppError> {
    Ok("Authorized".into())
}

#[derive(Clone)]
pub struct AppState {
    pub jwt: jwt::Config,
}

impl FromRef<AppState> for jwt::Config {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.jwt.clone()
    }
}

pub enum AppError {
    InternalServerError(Box<dyn StdError>),
}

impl From<TokenError> for AppError {
    fn from(error: TokenError) -> Self {
        AppError::InternalServerError(error.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::InternalServerError(error) => {
                tracing::error!("{error}");
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http::header;
    use http_body_util::BodyExt;
    use jsonwebtoken::{DecodingKey, EncodingKey};
    use tower::ServiceExt;

    use super::*;

    fn state() -> AppState {
        AppState {
            jwt: jwt::Config {
                expiration: Duration::from_secs(3600),
                encoding_key: EncodingKey::from_secret("secret".as_bytes()),
                decoding_key: DecodingKey::from_secret("secret".as_bytes()),
            },
        }
    }

    #[tokio::test]
    async fn test_authorization() {
        let app = app(state());

        let response = app
            .clone()
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let token = response.into_body().collect().await.unwrap().to_bytes();
        let token = String::from_utf8(token.into()).unwrap();

        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header(header::AUTHORIZATION, format!("Token {token}"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
