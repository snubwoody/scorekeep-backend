mod api;
mod error;
pub mod games;

use crate::api::Api;
pub use error::{Error, Result};
use poem::listener::TcpListener;
use poem::{Route, Server};
use poem_openapi::OpenApiService;
use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
}

/// Contains common resources such as database connections. Create
/// one and use it for the whole app.
#[derive(Clone)]
pub struct State {
    pool: sqlx::PgPool,
}

impl State {
    pub async fn new() -> Result<Self> {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(15)
            .connect(&db_url)
            .await?;

        Ok(Self { pool })
    }

    /// Create a state object with a predefined pool
    pub fn with_pool(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &sqlx::PgPool {
        &self.pool
    }
}
pub async fn create_user(pool: &sqlx::PgPool) -> User {
    let row = sqlx::query!("INSERT INTO users DEFAULT VALUES RETURNING id")
        .fetch_one(pool)
        .await
        .unwrap();

    
    User { id: row.id }
}

/// Generate a random alphanumeric string with a specified length.
///
/// # Example
/// ```
/// use scorekeep::gen_random_string;
///
/// let s = gen_random_string(10);
/// assert_eq!(s.len(),10);
/// ```
pub fn gen_random_string(length: usize) -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub async fn router(state: State) -> Result<Route> {
    let api = Api::new(state.clone());
    let games_api = games::GamesApi::new(state);

    let api_service = OpenApiService::new((api, games_api), "Scorekeep API", "1.0");
    let ui = api_service.scalar();
    let app = Route::new().nest("/api/v1", api_service).nest("/docs", ui);

    Ok(app)
}

pub async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    tracing_subscriber::fmt::init();
    let state = State::new().await?;
    let app = router(state).await?;
    let listener = TcpListener::bind("127.0.0.1:3000");
    info!("Listening for requests on port 3000");
    Server::new(listener).run(app).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_random_string() {
        let string = gen_random_string(6);
        dbg!(string);
    }

    #[sqlx::test]
    async fn add_user_to_db(pool: sqlx::PgPool) {
        let user = create_user(&pool).await;
        sqlx::query!("SELECT * FROM users WHERE id = $1", user.id)
            .fetch_one(&pool)
            .await
            .unwrap();
    }
}
