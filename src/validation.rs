use std::collections::HashMap;

use axum::async_trait;
use serde::Serialize;

use crate::{
    db::{get_user_by_email, get_user_by_username},
    Database,
};

#[async_trait]
pub trait Validate {
    async fn validate(&self, pool: &Database) -> Result<(), ValidationErrorsWrapper>;
}

#[derive(Debug)]
pub enum ValidationErrorsWrapper {
    ValidationErrors(ValidationErrors),
    DatabaseError(sqlx::Error),
}

impl From<ValidationErrors> for ValidationErrorsWrapper {
    fn from(value: ValidationErrors) -> Self {
        ValidationErrorsWrapper::ValidationErrors(value)
    }
}

impl From<sqlx::Error> for ValidationErrorsWrapper {
    fn from(value: sqlx::Error) -> Self {
        ValidationErrorsWrapper::DatabaseError(value)
    }
}

#[derive(Debug, Default, Serialize, Clone, PartialEq)]
pub struct ValidationErrors(HashMap<&'static str, Vec<&'static str>>);

impl ValidationErrors {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add(&mut self, field: &'static str, error: &'static str) {
        self.0
            .entry(field)
            .and_modify(|e| e.push(error))
            .or_insert_with(|| vec![error]);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub fn validate_not_empty(errors: &mut ValidationErrors, field: &'static str, value: &str) -> bool {
    if value.is_empty() {
        errors.add(field, "can't be blank");
        false
    } else {
        true
    }
}

pub async fn validate_unique_username(
    errors: &mut ValidationErrors,
    username: &str,
    pool: &Database,
) -> Result<bool, sqlx::Error> {
    if get_user_by_username(pool, username).await?.is_some() {
        errors.add("username", "has already been taken");
        Ok(false)
    } else {
        Ok(true)
    }
}

pub async fn validate_unique_email(
    errors: &mut ValidationErrors,
    email: &str,
    pool: &Database,
) -> Result<bool, sqlx::Error> {
    if get_user_by_email(pool, email).await?.is_some() {
        errors.add("email", "has already been taken");
        Ok(false)
    } else {
        Ok(true)
    }
}
