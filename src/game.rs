use crate::Error;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub id: Uuid,
    pub name: String,
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    id: Uuid,
    username: String,
    points: i32,
}

#[derive(Clone)]
pub struct GameService{
    pool: sqlx::PgPool,
}

impl GameService{
    pub fn new(pool: sqlx::PgPool) -> Self{
        Self{pool}
    }

    pub async fn create_game(&self,name: &str) -> Game {
        let row = sqlx::query!("INSERT INTO games(name) VALUES ($1) RETURNING *", name)
            .fetch_one(&self.pool)
            .await
            .unwrap();

        let game = Game {
            id: row.id,
            name: row.name,
            players: Vec::new(),
        };

        game
    }

    pub async fn get_game(&self, id: Uuid) -> Option<Game> {
        let result = sqlx::query!("SELECT * FROM games WHERE id = $1", id)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(row) => {
                let game = Game {
                    id: row.id,
                    name: row.name,
                    players: Vec::new(),
                };

                Some(game)
            }
            Err(e) => None,
        }
    }

    /// Add a player to a game
    pub async fn add_player(&self,game_id: Uuid, user_id: Uuid) -> crate::Result<()> {
        sqlx::query!(
            "INSERT INTO game_participants(game,player) VALUES ($1,$2)",
            game_id,
            user_id
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    /// Set the game participants points
    pub async fn set_points(
        &self,
        game_id: Uuid,
        user_id: Uuid,
        points: i32,
    ) -> crate::Result<()> {
        sqlx::query!(
        "UPDATE game_participants SET points = $1 WHERE game = $2 AND player = $3",
        points,
        game_id,
        user_id
    )
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}


