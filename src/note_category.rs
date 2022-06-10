use actix_web::web;
use sqlx::{query, query_as, FromRow, Error, postgres::PgPool};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteCategory{
    pub id: i32,
    pub note_id: i32,
    pub category_id: i32,
}

impl NoteCategory{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<NoteCategory>, Error>{
        let notes_categories = query_as!(NoteCategory, r#"SELECT id, note_id, category_id FROM notes_categories"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(notes_categories)
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<NoteCategory, Error>{
        let note_category = query_as!(NoteCategory, r#"SELECT id, note_id, category_id FROM notes_categories WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(note_category)
    }

    pub async fn get_last_inserted(pool: web::Data<PgPool>) -> Result<NoteCategory, Error>{
        let note_category = query_as!(NoteCategory, r#"SELECT id, note_id, category_id FROM notes_categories WHERE id=(SELECT CURRVAL(PG_GET_SERIAL_SEQUENCE('notes_categories', 'id')))"#)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(note_category)
    }

    pub async fn new(pool: web::Data<PgPool>, note_id: i32, category_id: i32) -> Result<NoteCategory, Error>{
        let id = query("INSERT INTO notes_categories (note_id, category_id) VALUES (?, ?);")
            .bind(note_id)
            .bind(category_id)
            .execute(pool.get_ref())
            .await?;
        Ok(Self::get_last_inserted(pool).await?)
    }
}

