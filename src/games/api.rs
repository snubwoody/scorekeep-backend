use super::{Game, GameService};
use crate::State;
use crate::api::ErrorResponse;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::{ApiResponse, Object, OpenApi};
use poem_openapi::param::Path;
use serde::Serialize;
use uuid::Uuid;

#[derive(Object,Serialize)]
pub struct CreateGameRequest{
    pub name: String,
}

#[derive(ApiResponse)]
enum GetGamesResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Game>>),
    /// An unknown error occurred
    #[oai(status = 500)]
    Unknown(Json<ErrorResponse>),
}

#[derive(ApiResponse)]
enum CreateGameResponse {
    /// Game created successfully
    #[oai(status = 201)]
    Ok(Json<Game>),
    /// An unknown error occurred
    #[oai(status = 500)]
    Unknown(Json<ErrorResponse>),
}


#[derive(ApiResponse)]
enum GetGameCodeResponse {
    /// Game created successfully
    #[oai(status = 200)]
    Ok(PlainText<String>),
    /// A game with the corresponding id was not found
    #[oai(status = 404)]
    NotFound,
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
    /// Get all the games that a user is part of
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

    /// Create a new game
    #[oai(path = "/game", method = "post")]
    async fn create_game(&self,req: Json<CreateGameRequest>) -> CreateGameResponse {
        let result = self.game_service.create_game(&req.name).await;
        match result {
            Ok(games) => CreateGameResponse::Ok(Json(games)),
            Err(e) => {
                let response = ErrorResponse::new(&e.to_string());
                CreateGameResponse::Unknown(Json(response))
            }
        }
    }

    /// Generate a join code for a game
    #[oai(path="/game/:id/code",method="post")]
    async fn gen_game_code(&self,id: Path<Uuid>) -> GetGameCodeResponse {
        let result = self.game_service.create_code(*id).await;

        match result {
            Ok(code) => {
                GetGameCodeResponse::Ok(PlainText(code))
            },
            Err(e) => {
                let response = ErrorResponse::new(&e.to_string());
                GetGameCodeResponse::Unknown(Json(response))
            }
        }
    }
}
