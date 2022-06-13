use actix_web::web;
use sqlx::{query, query_as, FromRow, Error, postgres::PgPool};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteLabel{
    pub id: i32,
    pub note_id: i32,
    pub label_id: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewNoteLabel{
    pub note_id: i32,
    pub label_id: i32,
}

impl NoteLabel{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<NoteLabel>, Error>{
        query_as!(NoteLabel, r#"SELECT id, note_id, label_id FROM notes_labels"#)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<NoteLabel, Error>{
        query_as!(NoteLabel, r#"SELECT id, note_id, label_id FROM notes_labels WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_last_inserted(pool: web::Data<PgPool>) -> Result<NoteLabel, Error>{
        query_as!(NoteLabel, r#"SELECT id, note_id, label_id FROM notes_labels WHERE id=(SELECT CURRVAL(PG_GET_SERIAL_SEQUENCE('notes_labels', 'id')))"#)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, note_id: i32, label_id: i32) -> Result<NoteLabel, Error>{
        query_as!(NoteLabel, r#"INSERT INTO notes_labels (note_id, label_id) VALUES ($1, $2) RETURNING id, note_id, label_id;"#, note_id, label_id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, note_label: NoteLabel) -> Result<NoteLabel, Error>{
        query_as!(NoteLabel, r#"UPDATE notes_labels SET note_id=$2, label_id=$3 WHERE id=$1 RETURNING id, note_id, label_id;"#, note_label.id, note_label.note_id, note_label.label_id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<NoteLabel, Error>{
        query_as!(NoteLabel, r#"DELETE FROM notes_labels WHERE id = $1 RETURNING id, note_id, label_id;"#, id)
            .fetch_one(pool.get_ref())
            .await
    }
}

