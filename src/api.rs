use crate::State;
use crate::game::{Game, GameService};
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, Object, OpenApi};
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

#[derive(ApiResponse)]
enum GetGamesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Game>>),
    /// An unknown error occurred
    #[oai(status = 500)]
    Unknown(Json<ErrorResponse>),
}

pub struct Api {
    state: State,
    game_service: GameService,
}

impl Api {
    pub async fn new(state: State) -> crate::Result<Self> {
        let game_service = GameService::new(state.clone());

        Ok(Self {
            state,
            game_service,
        })
    }
}

#[OpenApi]
impl Api {
    #[oai(path = "/health", method = "get")]
    async fn health(&self) {}

    /// Get all the games that a user is part of.
    #[oai(path = "/games", method = "get")]
    async fn get_games(&self) -> GetGamesResponse {
        let result = self.game_service.get_all_games().await;
        match result {
            Ok(games) => GetGamesResponse::Ok(Json(games)),
            Err(e) => {
                let response = ErrorResponse::new(&e.to_string());
                GetGamesResponse::Unknown(Json(response))
            }
        }
    }
}
