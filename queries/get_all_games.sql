SELECT
    games.id as game_id,
    games.name as game_name,
    p.player as player_id,
    p.username,
    p.joined_at,
    p.points
FROM games
LEFT JOIN game_participants p ON games.id = p.game;