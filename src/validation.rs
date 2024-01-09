use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
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

    pub fn into_result(self) -> Result<(), ValidationErrors> {
        if self.is_empty() {
            Ok(())
        } else {
            Err(self)
        }
    }
}

pub fn validate_not_empty(errors: &mut ValidationErrors, field: &'static str, value: &str) -> bool {
    if value.is_empty() {
        errors.add(field, "not_empty");
        false
    } else {
        true
    }
}
