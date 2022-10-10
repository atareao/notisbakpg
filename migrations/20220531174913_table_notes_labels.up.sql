-- Add up migration script here
-- Add up migration script here
CREATE TABLE IF NOT EXISTS notes_labels(
    id SERIAL PRIMARY KEY NOT NULL,
    note_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    UNIQUE(note_id, label_id)
);
