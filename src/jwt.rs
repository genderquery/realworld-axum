use std::{
    borrow::Cow,
    env,
    ops::Add,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header, request, HeaderMap},
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Clone)]
pub struct Config {
    pub expiration: Duration,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("{0}")]
    Required(&'static str),
    #[error("{0}")]
    InvalidValue(&'static str),
}

impl Config {
    pub fn try_from_env() -> Result<Self, ConfigError> {
        let secret =
            env::var("JWT_SECRET").map_err(|_| ConfigError::Required("JWT_SECRET must be set"))?;

        let encoding_key = EncodingKey::from_base64_secret(&secret)
            .unwrap_or_else(|_| EncodingKey::from_secret(secret.as_bytes()));

        let decoding_key = DecodingKey::from_base64_secret(&secret)
            .unwrap_or_else(|_| DecodingKey::from_secret(secret.as_bytes()));

        let expiration: u64 = env::var("JWT_EXPIRATION_SECONDS")
            .map_or_else(|_| Ok(3600), |s| s.parse())
            .map_err(|_| ConfigError::InvalidValue("JWT_EXPIRATION_SECONDS must be an integer"))?;

        Ok(Self {
            expiration: Duration::from_secs(expiration),
            encoding_key,
            decoding_key,
        })
    }
}

pub fn create_token(config: &Config, username: &str, email: &str) -> Result<String, TokenError> {
    let exp: u64 = SystemTime::now()
        .add(config.expiration)
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let claims = Claims {
        exp,
        username: username.to_owned(),
        email: email.to_owned(),
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &config.encoding_key,
    )?;

    Ok(token)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,
    pub username: String,
    pub email: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    Config: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = TokenError;

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let keys = Config::from_ref(state);
        let token = extract_token_from_headers(&parts.headers).ok_or(TokenError::InvalidHeader)?;
        let token_data =
            jsonwebtoken::decode::<Claims>(token, &keys.decoding_key, &Validation::default())?;

        Ok(token_data.claims)
    }
}

fn extract_token_from_headers(headers: &HeaderMap) -> Option<&str> {
    let header = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    let (schema, token) = header.split_once(' ')?;
    if schema.eq_ignore_ascii_case("token") || schema.eq_ignore_ascii_case("bearer") {
        Some(token.trim())
    } else {
        None
    }
}

#[derive(Debug, Error)]
pub enum TokenError {
    #[error(transparent)]
    Token(#[from] jsonwebtoken::errors::Error),
    #[error("Invalid or missing Authorization header")]
    InvalidHeader,
}

impl IntoResponse for TokenError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            TokenError::InvalidHeader => (
                StatusCode::UNAUTHORIZED,
                Cow::from(TokenError::InvalidHeader.to_string()),
            ),
            TokenError::Token(error) => (StatusCode::UNAUTHORIZED, Cow::from(error.to_string())),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}
