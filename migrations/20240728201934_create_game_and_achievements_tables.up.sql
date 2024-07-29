-- Add up migration script here
CREATE TABLE game_achievements (
    id VARCHAR(255) NOT NULL,
    game_id INT NOT NULL,
    display_name VARCHAR(1023) NOT NULL,
    description TEXT NOT NULL,
    locked_image_url VARCHAR(1023),
    unlocked_image_url VARCHAR(1023),
    unlocked_date TIMESTAMP WITH TIME ZONE,
    global_unlocked_percentage REAL NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (id, game_id)
);

CREATE INDEX game_achievements_game_id_index ON game_achievements (game_id);

CREATE TABLE games (
    id INT NOT NULL PRIMARY KEY,
    name VARCHAR(1023) NOT NULL,
    header_image_url VARCHAR(1023) NOT NULL,
    playtime INT NOT NULL,
    last_played TIMESTAMP WITH TIME ZONE NOT NULL,
    link_url VARCHAR(1023) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);
