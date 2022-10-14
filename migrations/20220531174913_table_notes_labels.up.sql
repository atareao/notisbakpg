CREATE TABLE IF NOT EXISTS notes_labels(
    id SERIAL PRIMARY KEY NOT NULL,
    note_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
    UNIQUE(note_id, label_id)
);
