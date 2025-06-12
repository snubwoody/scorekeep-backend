use sqlx::PgPool;
use scorekeep::game::{self,create_game};

#[sqlx::test]
async fn game_added_to_db(pool: PgPool){
    let game = create_game(&pool,"Movie Night").await;    
    sqlx::query!("SELECT * FROM games WHERE id = $1",game.id)
        .fetch_one(&pool)
        .await
        .unwrap();
}

#[sqlx::test]
async fn get_game(pool: PgPool){
    let game = create_game(&pool,"Movie Night").await;
    let result = game::get_game(&pool,game.id).await;
    assert!(result.is_some());
}