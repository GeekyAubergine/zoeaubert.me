-- Add up migration script here
CREATE TABLE micro_posts (
    uuid UUID PRIMARY KEY,
    slug VARCHAR(255) NOT NULL,
    date TIMESTAMP WITH TIME ZONE NOT NULL,
    content TEXT NOT NULL,
    image_order UUID[] NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
)
