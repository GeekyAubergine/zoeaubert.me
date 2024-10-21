-- Add up migration script here
CREATE TABLE tags (
    entity_uuid UUID NOT NULL,
    tag VARCHAR(255) NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (entity_uuid, tag)
)
