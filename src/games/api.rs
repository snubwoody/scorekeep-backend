use super::{Game, GameService};
use crate::State;
use crate::api::ErrorResponse;
use poem_openapi::payload::Json;
use poem_openapi::{ApiResponse, OpenApi};

#[derive(ApiResponse)]
enum GetGamesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Game>>),
    /// An unknown error occurred
    #[oai(status = 500)]
    Unknown(Json<ErrorResponse>),
}

pub struct GamesApi {
    pub state: State,
    pub game_service: GameService,
}

impl GamesApi {
    pub fn new(state: State) -> Self {
        let game_service = GameService::new(state.clone());

        Self {
            state,
            game_service,
        }
    }
}

#[OpenApi]
impl GamesApi {
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
