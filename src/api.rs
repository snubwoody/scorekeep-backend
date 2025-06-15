use poem_openapi::{ApiResponse, OpenApi};
use poem_openapi::payload::Json;
use crate::game::{Game, GameService};
use crate::State;

#[derive(ApiResponse)]
enum GetGamesResponse{
    #[oai(status = 200)]
    Ok(Json<Vec<Game>>),
    /// An unknown error occurred
    #[oai(status = 500)]
    Unknown,
}

pub struct Api{
    state: State,
    game_service: GameService
}

impl Api{
    pub async fn new(state: State) -> crate::Result<Self> {
        let game_service = GameService::new(state.clone());

        Ok(Self{state,game_service})

    }
}

#[OpenApi]
impl Api{
    #[oai(path = "/health", method = "get")]
    async fn health(&self) {}

    #[oai(path="/games", method = "get")]
    async fn get_games(&self) -> GetGamesResponse{
        let result = self.game_service.get_all_games().await;
        match result { 
            Ok(games) => {
                GetGamesResponse::Ok(Json(games))
            },
            Err(e) => {
                GetGamesResponse::Unknown
            }
        }
    }
}