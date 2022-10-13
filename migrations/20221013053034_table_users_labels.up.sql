-- Add up migration script here
CREATE TABLE IF NOT EXISTS users_labels(
    id SERIAL PRIMARY KEY NOT NULL,
    user_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    UNIQUE(user_id, label_id)
);
