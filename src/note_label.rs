use actix_web::web;
use sqlx::{query, FromRow, Error, Row, postgres::{PgPool, PgRow}};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteLabel{
    pub id: i32,
    pub note_id: i32,
    pub label_id: i32,
}

impl NoteLabel{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<NoteLabel>, Error>{
        query(r#"SELECT id, note_id, label_id FROM notes_labels"#)
            .map(|row: PgRow| NoteLabel{
                id: row.get("id"),
                note_id: row.get("note_id"),
                label_id: row.get("label_id"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<NoteLabel, Error>{
        query(r#"SELECT id, note_id, label_id FROM notes_labels WHERE id = ?"#)
            .bind(id)
            .map(|row: PgRow| NoteLabel{
                id: row.get("id"),
                note_id: row.get("note_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_last_inserted(pool: web::Data<PgPool>) -> Result<NoteLabel, Error>{
        query(r#"SELECT id, note_id, label_id FROM notes_labels WHERE id=(SELECT CURRVAL(PG_GET_SERIAL_SEQUENCE('notes_labels', 'id')))"#)
            .map(|row: PgRow| NoteLabel{
                id: row.get("id"),
                note_id: row.get("note_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, note_id: i32, label_id: i32) -> Result<NoteLabel, Error>{
        query(r#"INSERT INTO notes_labels (note_id, label_id) VALUES ($1, $2) RETURNING id, note_id, label_id;"#)
            .bind(note_id)
            .bind(label_id)
            .map(|row: PgRow| NoteLabel{
                id: row.get("id"),
                note_id: row.get("note_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, note_label: NoteLabel) -> Result<NoteLabel, Error>{
        query(r#"UPDATE notes_labels SET note_id = ?, label_id = ? WHERE id = ?  RETURNING id, note_id, label_id;"#)
            .bind(note_label.note_id)
            .bind(note_label.label_id)
            .bind(note_label.id)
            .map(|row: PgRow| NoteLabel{
                id: row.get("id"),
                note_id: row.get("note_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<NoteLabel, Error>{
        query(r#"DELETE FROM notes_labels WHERE id = ? RETURNING id, note_id, label_id;"#)
            .bind(id)
            .map(|row: PgRow| NoteLabel{
                id: row.get("id"),
                note_id: row.get("note_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}

