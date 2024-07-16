-- Your SQL goes here
ALTER TABLE users
ADD COLUMN session_token VARCHAR NOT NULL DEFAULT '';