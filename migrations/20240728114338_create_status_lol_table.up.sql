-- Add up migration script here
CREATE TABLE status_lol_posts (
    id VARCHAR(127) NOT NULL PRIMARY KEY,
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    content TEXT NOT NULL,
    emoji VARCHAR(255) NOT NULL,
    original_url VARCHAR(255) NOT NULL
);
