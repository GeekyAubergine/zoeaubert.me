-- Add up migration script here
CREATE TABLE lego_sets (
    id INT NOT NULL PRIMARY KEY,
    name VARCHAR(1023) NOT NULL,
    number VARCHAR(255) NOT NULL,
    category VARCHAR(255) NOT NULL,
    pieces INT NOT NULL,
    image_url VARCHAR(1023) NOT NULL,
    thumbnail_url VARCHAR(1023) NOT NULL,
    link VARCHAR(1023) NOT NULL,
    quantity INT NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE lego_minifigs (
    id VARCHAR(255) NOT NULL PRIMARY KEY,
    name VARCHAR(1023) NOT NULL,
    category VARCHAR(255) NOT NULL,
    owned_in_sets INT NOT NULL,
    owned_loose INT NOT NULL,
    total_owned INT NOT NULL,
    image_url VARCHAR(1023) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
)
