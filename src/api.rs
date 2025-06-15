use poem_openapi::OpenApi;
use crate::game::GameService;
use crate::State;

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
    #[oai(path = "/api/v1/health", method = "get")]
    async fn health(&self) {}

    #[oai(path="/api/v1/games", method = "get")]
    async fn get_games(&self){

    }
}