pub mod api;

use poem_openapi::Object;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
pub use api::AuthApi;

#[derive(Serialize, Deserialize,Object)]
pub struct User {
    pub id: Uuid,
}

pub async fn create_user(pool: &sqlx::PgPool) -> User {
    let row = sqlx::query!("INSERT INTO users DEFAULT VALUES RETURNING id")
        .fetch_one(pool)
        .await
        .unwrap();

    User { id: row.id }
}

#[cfg(test)]
mod tests{
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