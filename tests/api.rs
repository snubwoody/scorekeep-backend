use http::StatusCode;
use poem::test::TestClient;
use scorekeep::games::{Game, GameService};
use scorekeep::{State, router};
use sqlx::PgPool;

#[sqlx::test]
async fn health_check(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool);
    let app = router(state).await?;
    let cli = TestClient::new(app);
    let response = cli.get("/api/v1/health").send().await;
    response.assert_status(StatusCode::OK);
    Ok(())
}

#[sqlx::test]
async fn get_game(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool);
    let games_service = GameService::new(state.clone());
    games_service.create_game("My game").await?;

    let app = router(state).await?;
    let cli = TestClient::new(app);
    let response = cli.get("/api/v1/games").send().await;

    response.assert_status(StatusCode::OK);
    let body = response.json().await;
    let games: Vec<Game> = body.value().deserialize();
    assert_eq!(games.len(), 1);

    Ok(())
}
