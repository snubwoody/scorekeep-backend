use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game{
    pub id: Uuid,
    pub name: String,
    pub players: Vec<Player>,
}

#[derive(Serialize,Deserialize)]
struct User{
    id: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
struct Player{
    id: Uuid,
    username: String,
    points: i32,
}

struct State{
    pool: sqlx::PgPool
}

impl State{
    pub async fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();

        Self{
            pool
        }
    }
}
async fn create_user(pool: &sqlx::PgPool) -> User{
    let row = sqlx::query!("INSERT INTO users DEFAULT VALUES RETURNING id")
        .fetch_one(pool)
        .await
        .unwrap();
    
    let user = User{id:row.id};
    user
}

pub async fn create_game(pool: &sqlx::PgPool,name:&str) -> Game {
    let row = sqlx::query!("INSERT INTO games(name) VALUES ($1) RETURNING *",name)
        .fetch_one(pool)
        .await
        .unwrap();
    
    let game = Game{
        id: row.id,
        name: row.name,
        players: Vec::new()
    };
    
    game
}

pub async fn get_game(pool: &sqlx::PgPool,id:Uuid) -> Option<Game> {
    let result = sqlx::query!("SELECT * FROM games WHERE id = $1",id)
        .fetch_one(pool)
        .await;
    
    match result {  
        Ok(row) => {
            let game = Game{
                id: row.id,
                name: row.name,
                players: Vec::new()
            };
            
            Some(game)
        },Err(e) => {
            None
        }
    }    
}  

#[cfg(test)]
mod tests {
    use super::*;
    
    #[sqlx::test]
    async fn add_user_to_db(pool: sqlx::PgPool) {
        let user = create_user(&pool).await;
        sqlx::query!("SELECT * FROM users WHERE id = $1", user.id)
            .fetch_one(&pool)
            .await
            .unwrap();
    }
}
