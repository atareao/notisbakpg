use actix_web::web;
use sqlx::{query, FromRow, Error, Row, postgres::{PgPool, PgRow}};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct NoteCategory{
    pub id: i32,
    pub note_id: i32,
    pub category_id: i32,
}

impl NoteCategory{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<NoteCategory>, Error>{
        query(r#"SELECT id, note_id, category_id FROM notes_categories"#)
            .map(|row: PgRow| NoteCategory{
                id: row.get("id"),
                note_id: row.get("note_id"),
                category_id: row.get("category_id"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<NoteCategory, Error>{
        query(r#"SELECT id, note_id, category_id FROM notes_categories WHERE id = ?"#)
            .bind(id)
            .map(|row: PgRow| NoteCategory{
                id: row.get("id"),
                note_id: row.get("note_id"),
                category_id: row.get("category_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_last_inserted(pool: web::Data<PgPool>) -> Result<NoteCategory, Error>{
        query(r#"SELECT id, note_id, category_id FROM notes_categories WHERE id=(SELECT CURRVAL(PG_GET_SERIAL_SEQUENCE('notes_categories', 'id')))"#)
            .map(|row: PgRow| NoteCategory{
                id: row.get("id"),
                note_id: row.get("note_id"),
                category_id: row.get("category_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, note_id: i32, category_id: i32) -> Result<NoteCategory, Error>{
        query(r#"INSERT INTO notes_categories (note_id, category_id) VALUES ($1, $2) RETURNING id, note_id, category_id;"#)
            .bind(note_id)
            .bind(category_id)
            .map(|row: PgRow| NoteCategory{
                id: row.get("id"),
                note_id: row.get("note_id"),
                category_id: row.get("category_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
    pub async fn delete(pool: web::Data<PgPool>, note_id: i32, category_id: i32) -> Result<NoteCategory, Error>{
        query(r#"DELETE FROM notes_categories WHERE note_id = $1 AND category_id = $2 RETURNING id, note_id, category_id;"#)
            .bind(note_id)
            .bind(category_id)
            .map(|row: PgRow| NoteCategory{
                id: row.get("id"),
                note_id: row.get("note_id"),
                category_id: row.get("category_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}

