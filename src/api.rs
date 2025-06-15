use std::env;
use poem_openapi::OpenApi;
use sqlx::postgres::PgPoolOptions;
use crate::game::GameService;

pub struct Api{
    game_service: GameService
}

impl Api{
    pub async fn new() -> crate::Result<Self> {
        let url = env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(15)
            .connect(&url)
            .await?;

        let game_service = GameService::new(pool);

        Ok(Self{game_service})

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