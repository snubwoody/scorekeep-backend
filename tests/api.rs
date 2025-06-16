use http::StatusCode;
use poem::test::TestClient;
use poem_openapi::payload::Json;
use scorekeep::games::{Game, GameService};
use scorekeep::{State, router};
use sqlx::PgPool;
use scorekeep::auth::User;
use scorekeep::games::api::CreateGameRequest;

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

#[sqlx::test]
async fn create_new_game(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool);
    let games_service = GameService::new(state.clone());
    games_service.create_game("My game").await?;

    
    let app = router(state).await?;
    let cli = TestClient::new(app);
    
    let body = CreateGameRequest{name: "My game".to_owned()};
    let response = cli.post("/api/v1/game")
        .body_json(&body)
        .send().await;

    response.assert_status(StatusCode::CREATED);
    let body = response.json().await;
    let game: Game = body.value().deserialize();
    assert_eq!(game.name, "My game");
    Ok(())
}

#[sqlx::test]
async fn create_new_user(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool);
    let app = router(state).await?;
    let cli = TestClient::new(app);
    
    let response = cli.post("/api/v1/auth/signup")
        .send().await;

    response.assert_status(StatusCode::CREATED);
    let body = response.json().await;
    let _: User = body.value().deserialize();
    Ok(())
}
