-- Add up migration script here
CREATE TABLE IF NOT EXISTS notes(
    id SERIAL PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    user_id INTEGER NOT NULL,
    UNIQUE (title, user_id)
);
