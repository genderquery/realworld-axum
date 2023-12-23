use std::collections::HashMap;

use serde::Serialize;

use crate::AppState;

pub trait Validate {
    fn validate(&self) -> Result<(), ValidationErrors>;
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

pub fn validate_not_empty(errors: &mut ValidationErrors, field: &'static str, value: &str) {
    if value.is_empty() {
        errors.add(field, "can't be blank");
    }
}

pub fn validate_unique_username(state: &AppState, username: &str) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    if state.db.read().unwrap().contains_key(username) {
        errors.add("username", "has already been taken");
    }

    errors.is_empty().then_some(()).ok_or(errors)
}

pub fn validate_unique_email(state: &AppState, email: &str) -> Result<(), ValidationErrors> {
    let mut errors = ValidationErrors::new();

    let db = state.db.read().unwrap();
    if db.values().any(|(_, e, _)| e == email) {
        errors.add("email", "has already been taken");
    }

    errors.is_empty().then_some(()).ok_or(errors)
}
