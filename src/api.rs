use crate::State;
use poem_openapi::{Object, OpenApi};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Object)]
pub struct ErrorResponse {
    message: String,
    #[oai(skip_serializing_if_is_none)]
    details: Option<String>,
}

impl ErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            details: None,
        }
    }

    pub fn with_details(message: &str, details: &str) -> Self {
        Self {
            message: message.to_owned(),
            details: Some(details.to_owned()),
        }
    }
}

pub struct Api {
    pub state: State,
}

impl Api {
    pub fn new(state: State) -> Self {
        Self { state }
    }
}

#[OpenApi]
impl Api {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) {}
}
