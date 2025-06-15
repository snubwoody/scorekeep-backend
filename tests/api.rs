use poem::test::TestClient;
use sqlx::PgPool;
use scorekeep::{router, State};

#[sqlx::test]
async fn get_game(pool: PgPool) -> scorekeep::Result<()>{
    let state = State::with_pool(pool);
    let app = router(state).await?;
    let cli = TestClient::new(app);
    let response = cli.get("/api/v1/games").send().await;
    // let body = response.json().await;
    // dbg!(body);
    Ok(())
}