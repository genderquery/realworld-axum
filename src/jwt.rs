use std::{error::Error, fmt::Display, sync::Arc};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header, request, StatusCode},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,
}

#[derive(Clone)]
pub struct Config {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub duration_secs: u64,
}

#[derive(Debug)]
pub struct ConfigError(&'static str);

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

impl Error for ConfigError {}

impl Config {
    pub fn try_from_env() -> Result<Self, ConfigError> {
        let secret =
            std::env::var("JWT_SECRET").map_err(|_| ConfigError("JWT_SECRET must be set"))?;

        let encoding_key = EncodingKey::from_base64_secret(&secret)
            .unwrap_or_else(|_| EncodingKey::from_secret(secret.as_bytes()));

        let decoding_key = DecodingKey::from_base64_secret(&secret)
            .unwrap_or_else(|_| DecodingKey::from_secret(secret.as_bytes()));

        let duration_secs: u64 = std::env::var("JWT_DURATION_SECS")
            .map_or_else(|_| Ok(3600), |s| s.parse())
            .map_err(|_| ConfigError("JWT_DURATION_SECS must be an integer"))?;

        Ok(Self {
            duration_secs,
            encoding_key,
            decoding_key,
        })
    }
}

#[derive(Clone)]
pub struct Jwt {
    // Arc is cheaper to clone than Config
    config: Arc<Config>,
}

impl Jwt {
    pub fn new(config: Config) -> Self {
        Jwt {
            config: Arc::new(config),
        }
    }

    pub fn encode(&self) -> Result<String, jsonwebtoken::errors::Error> {
        let now = jsonwebtoken::get_current_timestamp();
        let exp = self.config.duration_secs.saturating_add(now);
        let claims = Claims { exp };
        jsonwebtoken::encode(&Header::default(), &claims, &self.config.encoding_key)
    }

    pub fn decode(&self, token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<Claims>(token, &self.config.decoding_key, &Validation::default())
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    Jwt: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let jwt = Jwt::from_ref(state);
        let token = extract_token(parts).ok_or(StatusCode::UNAUTHORIZED)?;
        let token = jwt.decode(token).map_err(|_| StatusCode::UNAUTHORIZED)?;
        Ok(token.claims)
    }
}

fn extract_token(parts: &mut request::Parts) -> Option<&str> {
    let value = parts.headers.get(header::AUTHORIZATION)?;
    let (schema, token) = value.to_str().ok()?.split_once(' ')?;
    if schema.eq_ignore_ascii_case("bearer") || schema.eq_ignore_ascii_case("token") {
        Some(token.trim())
    } else {
        None
    }
}
