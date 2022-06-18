-- Add up migration script here
CREATE TABLE IF NOT EXISTS categories(
    id SERIAL PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    UNIQUE (name, user_id)
);
