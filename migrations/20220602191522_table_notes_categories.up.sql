CREATE TABLE IF NOT EXISTS notes_categories(
    id SERIAL PRIMARY KEY NOT NULL,
    note_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    UNIQUE(note_id, category_id)
);
