-- Add up migration script here
CREATE TABLE album_photos (
    uuid UUID PRIMARY KEY,
    album_uuid UUID NOT NULL,
    small_image_uuid UUID NOT NULL,
    large_image_uuid UUID NOT NULL,
    full_image_uuid UUID NOT NULL,
    file_name VARCHAR(1023) NOT NULL,
    featured BOOLEAN NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
)
