use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request, StatusCode},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Config {
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
    pub duration_secs: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: u64,
}

#[derive(Clone)]
pub struct Jwt {
    config: Config,
}

impl Jwt {
    pub fn new(config: Config) -> Self {
        Jwt { config }
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
