use std::collections::HashMap;

use serde::Serialize;

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
