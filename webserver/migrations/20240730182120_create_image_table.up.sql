-- Add up migration script here
CREATE TABLE images (
    uuid UUID NOT NULL PRIMARY KEY,
    url VARCHAR(1023) NOT NULL,
    alt TEXT NOT NULL,
    width INT NOT NULL,
    height INT NOT NULL,
    title TEXT,
    description TEXT,
    date TIMESTAMP WITH TIME ZONE,
    parent_permalink VARCHAR(1023),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
)
