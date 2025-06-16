mod api;

use crate::{State, gen_random_string};
use chrono::{DateTime, Duration, Utc};
use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub use api::GamesApi;

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug, Object)]
pub struct Player {
    pub id: Uuid,
    pub username: String,
    pub points: i32,
    pub joined_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct GameService {
    state: State,
}

impl GameService {
    pub fn new(state: State) -> Self {
        Self { state }
    }

    /// Get a game from the database.
    pub async fn get_game(&self, id: Uuid) -> crate::Result<Option<Game>> {
        let rows = sqlx::query!("SELECT * FROM game_participants WHERE game = $1", id)
            .fetch_all(self.state.pool())
            .await?;

        let players: Vec<Player> = rows
            .into_iter()
            .map(|row| Player {
                id: row.player,
                joined_at: row.joined_at,
                username: row.username,
                points: row.points,
            })
            .collect();

        let result = sqlx::query!("SELECT * FROM games WHERE id = $1", id)
            .fetch_one(self.state.pool())
            .await;

        match result {
            Ok(row) => {
                let game = Game {
                    id: row.id,
                    name: row.name,
                    players,
                };

                Ok(Some(game))
            }
            // FIXME: handle the error
            Err(e) => Ok(None),
        }
    }

    /// Get all games from the database.
    pub async fn get_all_games(&self) -> crate::Result<Vec<Game>> {
        let rows = sqlx::query!("SELECT id FROM games")
            .fetch_all(self.state.pool())
            .await?;

        let mut games = vec![];
        for row in rows {
            // Unwrapping because we know these games exist
            let game = self.get_game(row.id).await?.unwrap();
            games.push(game);
        }

        Ok(games)
    }

    /// Join a game using its 6 character game code.
    pub async fn join_game(&self, user_id: Uuid, code: &str) -> crate::Result<()> {
        sqlx::query!(
            "INSERT INTO game_participants(game,player)
            SELECT game, $1 FROM game_codes WHERE code = $2",
            user_id,
            code
        )
            .execute(self.state.pool())
            .await?;

        Ok(())
    }

    /// Create a 6 character alphanumeric code that users can use
    /// to join a game.
    pub async fn create_code(&self, game_id: Uuid) -> crate::Result<String> {
        // With a 6 length alphanumeric code,
        // there's about 56 billion variations so collision
        // isn't likely
        let code = gen_random_string(6);
        let expiry = Utc::now() + Duration::days(7);
        let row = sqlx::query!(
            "
            INSERT INTO game_codes(code,expires_at,game)
            VALUES($1,$2,$3)
            RETURNING code
            ",
            code,
            expiry,
            game_id,
        )
            .fetch_one(self.state.pool())
            .await?;

        Ok(row.code)
    }

    /// Create a new game.
    pub async fn create_game(&self, name: &str) -> crate::Result<Game> {
        let row = sqlx::query!("INSERT INTO games(name) VALUES ($1) RETURNING *", name)
            .fetch_one(self.state.pool())
            .await?;

        let game = Game {
            id: row.id,
            name: row.name,
            players: Vec::new(),
        };

        Ok(game)
    }

    /// Add a player to a game
    pub async fn add_player(&self, game_id: Uuid, user_id: Uuid) -> crate::Result<()> {
        sqlx::query!(
            "INSERT INTO game_participants(game,player) VALUES ($1,$2)",
            game_id,
            user_id
        )
            .execute(self.state.pool())
            .await?;

        Ok(())
    }

    /// Set the game participants points
    pub async fn set_points(&self, game_id: Uuid, user_id: Uuid, points: i32) -> crate::Result<()> {
        sqlx::query!(
            "UPDATE game_participants SET points = $1 WHERE game = $2 AND player = $3",
            points,
            game_id,
            user_id
        )
            .execute(self.state.pool())
            .await?;

        Ok(())
    }
}
