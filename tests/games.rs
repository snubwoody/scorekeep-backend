use scorekeep::games::GameService;
use scorekeep::{State, create_user};
use sqlx::PgPool;

#[sqlx::test]
async fn players_included_in_game(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let user1 = create_user(&pool).await;
    let user2 = create_user(&pool).await;

    let games = GameService::new(state);
    let game = games.create_game("").await?;
    games.add_player(game.id, user1.id).await?;
    games.add_player(game.id, user2.id).await?;

    let game = games.get_game(game.id).await?.unwrap();

    assert_eq!(game.players.len(), 2);
    Ok(())
}

#[sqlx::test]
async fn get_all_games(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let user1 = create_user(&pool).await;
    let user2 = create_user(&pool).await;

    let games = GameService::new(state);
    let game1 = games.create_game("Game 1").await?;
    let _ = games.create_game("Game 2").await?;
    let game3 = games.create_game("Game 3").await?;

    games.add_player(game1.id, user1.id).await?;
    games.add_player(game1.id, user2.id).await?;
    games.add_player(game3.id, user2.id).await?;

    let games = games.get_all_games().await?;
    let game1 = &games[0];
    let game2 = &games[1];
    let game3 = &games[2];

    assert_eq!(game1.players.len(), 2);
    assert!(game2.players.is_empty());
    assert_eq!(game3.players.len(), 1);
    Ok(())
}

#[sqlx::test]
async fn create_game_code(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let games = GameService::new(state);
    let game = games.create_game("").await?;
    let code = games.create_code(game.id).await?;

    let row = sqlx::query!("SELECT * FROM game_codes WHERE code = $1", code)
        .fetch_one(&pool)
        .await?;

    assert_eq!(row.game, game.id);

    Ok(())
}

#[sqlx::test]
async fn join_game(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let games = GameService::new(state);
    let game = games.create_game("").await?;
    let code = games.create_code(game.id).await?;
    let user = create_user(&pool).await;
    games.join_game(user.id, &code).await?;

    let rows = sqlx::query!("SELECT * FROM game_participants")
        .fetch_all(&pool)
        .await?;

    assert_eq!(rows.len(), 1);

    Ok(())
}

#[sqlx::test]
async fn game_code_is_6_characters(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let games = GameService::new(state);
    let game = games.create_game("").await?;
    let code = games.create_code(game.id).await?;
    assert_eq!(code.len(), 6);
    Ok(())
}

#[sqlx::test]
async fn game_added_to_db(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let games = GameService::new(state);
    let game = games.create_game("Movie Night").await?;
    sqlx::query!("SELECT * FROM games WHERE id = $1", game.id)
        .fetch_one(&pool)
        .await?;
    Ok(())
}

#[sqlx::test]
async fn get_game(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool);
    let games = GameService::new(state);
    let game = games.create_game("Movie Night").await?;
    let result = games.get_game(game.id).await?;
    assert!(result.is_some());
    Ok(())
}

#[sqlx::test]
async fn add_player_to_game(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let games = GameService::new(state);
    let user = create_user(&pool).await;
    let game = games.create_game("My game").await?;
    games.add_player(game.id, user.id).await?;

    let rows = sqlx::query!("SELECT * FROM game_participants WHERE game = $1", game.id)
        .fetch_all(&pool)
        .await?;

    assert_eq!(rows.len(), 1);
    Ok(())
}

#[sqlx::test]
async fn set_players_points(pool: PgPool) -> scorekeep::Result<()> {
    let state = State::with_pool(pool.clone());
    let games = GameService::new(state);
    let user = create_user(&pool).await;
    let game = games.create_game("My game").await?;
    games.add_player(game.id, user.id).await?;
    games.set_points(game.id, user.id, 100).await?;

    let row = sqlx::query!("SELECT * FROM game_participants WHERE player = $1", user.id)
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.points, 100);
    Ok(())
}
