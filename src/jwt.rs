use async_trait::async_trait;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request, StatusCode},
};
use axum_macros::FromRef;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
}

#[derive(FromRef, Clone)]
pub struct JwtState {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtState {
    pub fn from_secret(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    JwtState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let jwt = JwtState::from_ref(state);
        let token = extract_token(parts).ok_or(StatusCode::UNAUTHORIZED)?;
        let token = decode::<Claims>(token, &jwt.decoding_key, &Validation::default())
            .map_err(|_| StatusCode::UNAUTHORIZED)?;
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
