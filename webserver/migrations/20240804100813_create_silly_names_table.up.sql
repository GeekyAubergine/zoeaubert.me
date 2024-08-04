-- Add up migration script here
CREATE TABLE silly_names (
  uuid UUID PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  deleted_at TIMESTAMP WITH TIME ZONE
);
