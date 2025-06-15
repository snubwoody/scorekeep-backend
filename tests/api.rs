use http::StatusCode;
use poem::test::TestClient;
use sqlx::PgPool;
use scorekeep::{router, State};

#[sqlx::test]
async fn health_check(pool: PgPool) -> scorekeep::Result<()>{
    let state = State::with_pool(pool);
    let app = router(state).await?;
    let cli = TestClient::new(app);
    let response = cli.get("/api/v1/health").send().await;
    response.assert_status(StatusCode::OK);
    Ok(())
}


#[sqlx::test]
async fn get_game(pool: PgPool) -> scorekeep::Result<()>{
    let state = State::with_pool(pool);
    let app = router(state).await?;
    let cli = TestClient::new(app);
    let response = cli.get("/api/v1/games").send().await;
    // let body = response.json().await;
    response.assert_status(StatusCode::OK);
    Ok(())
}