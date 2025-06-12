CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4()
);

CREATE TABLE games(
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name TEXT NOT NULL DEFAULT 'My Game'
);

CREATE TABLE game_codes(
    code TEXT PRIMARY KEY,
    expires_at TIMESTAMPTZ NOT NULL,
    game UUID NOT NULL REFERENCES games(id)
);

CREATE TABLE game_participants(
    game UUID NOT NULL REFERENCES games(id),
    player UUID NOT NULL REFERENCES users(id),
    username TEXT NOT NULL DEFAULT  'Player',
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    points INT NOT NULL DEFAULT 0,
    -- A player can only participate in a game once
    PRIMARY KEY (game,player)
);