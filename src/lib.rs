use sqlx::PgPool;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

struct Game{
    id: Uuid,
    name: String,
    players: Vec<Player>,
}

#[derive(Serialize,Deserialize)]
struct User{
    id: Uuid,
}

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

async fn create_game(pool: &sqlx::PgPool,name:&str) {
    sqlx::query!("INSERT INTO games(name) VALUES ($1) RETURNING *",name);    
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
