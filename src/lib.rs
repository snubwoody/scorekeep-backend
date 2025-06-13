mod error;
pub mod game;

pub use error::{Error, Result};
use rand::Rng;
use rand::distr::Alphanumeric;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
}

pub struct State {
    pool: sqlx::PgPool,
}

impl State {
    pub async fn new() -> Self {
        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .unwrap();

        Self { pool }
    }
}
pub async fn create_user(pool: &sqlx::PgPool) -> User {
    let row = sqlx::query!("INSERT INTO users DEFAULT VALUES RETURNING id")
        .fetch_one(pool)
        .await
        .unwrap();

    let user = User { id: row.id };
    user
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
