use std::convert::TryInto;

use actix_web::web;
use chrono::{NaiveDateTime, Utc};
use sqlx::{query, FromRow, Error, Row, postgres::{PgPool, PgRow, PgQueryResult}};
use serde::{Serialize, Deserialize};
use crate::{label::Label, category::Category};
use serde_json::Value;
use utoipa::ToSchema;

//https://github.com/juhaku/utoipa

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Note{
    pub id: i32,
    #[schema(example = "Titulo")]
    pub title: String,
    #[schema(example = "Contenido")]
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct NewNote{
    pub title: String,
    pub body: Option<String>,
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct UpdateNote{
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
}

impl Note{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<Note>, Error>{
        query(r#"SELECT id, title, body, created_at, updated_at FROM notes"#)
            .map(|row: PgRow| Note{
                id: row.get("id"),
                title: row.get("title"),
                body: row.get("body"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<Note, Error>{
        query(r#"SELECT id, title, body, created_at, updated_at FROM notes WHERE id = $1"#)
            .bind(id)
            .map(|row: PgRow| Note{
                id: row.get("id"),
                title: row.get("title"),
                body: row.get("body"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, note: NewNote) -> Result<Note, Error>{
        let title = note.title;
        let body = note.body.unwrap_or("".to_string());
        let created_at = Utc::now().naive_utc();
        let updated_at = Utc::now().naive_utc();
        query(r#"INSERT INTO notes (title, body, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING id, title, body, created_at, updated_at;"#,)
            .bind(title)
            .bind(body)
            .bind(created_at)
            .bind(updated_at)
            .map(|row: PgRow| Note{
                id: row.get("id"),
                title: row.get("title"),
                body: row.get("body"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, content: Value) -> Result<Note, Error>{
        let updated_at = Utc::now().naive_utc();
        let id: i32 = content.get("id").as_ref().unwrap().as_i64().unwrap().try_into().unwrap();
        let title_option = content.get("title");
        let body_option = content.get("body");
        let mut sql = query("");
        if title_option != None && body_option != None{
            sql = query(r#"UPDATE notes SET title = $1, body = $2, updated_at = $3 WHERE id = $4 RETURNING id, title, body, created_at, updated_at;"#)
                .bind(title_option.unwrap().as_str().unwrap())
                .bind(body_option.unwrap().as_str().unwrap());
        }else if title_option != None{ 
            sql = query(r#"UPDATE notes SET title = $1, updated_at = $2 WHERE id = $3 RETURNING id, title, body, created_at, updated_at;"#)
                .bind(title_option.unwrap().as_str().unwrap())
        }else if body_option != None{
            sql = query(r#"UPDATE notes SET body = $1, updated_at = $2 WHERE id = $3 RETURNING id, title, body, created_at, updated_at;"#)
                .bind(body_option.unwrap().as_str().unwrap())
        }
        sql.bind(updated_at)
            .bind(id)
            .map(|row: PgRow| Note{
                id: row.get("id"),
                title: row.get("title"),
                body: row.get("body"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<Note, Error>{
        query(r#"DELETE FROM notes WHERE id = $1 RETURNING id, title, body, created_at, updated_at;"#)
            .bind(id)
            .map(|row: PgRow| Note{
                id: row.get("id"),
                title: row.get("title"),
                body: row.get("body"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn set_label(self, pool: web::Data<PgPool>, label_id: i32) -> Result<PgQueryResult, Error>{
        query("INSERT INTO notes_labels (note_id, label_id) VALUES (?, ?);")
            .bind(self.id)
            .bind(label_id)
            .execute(pool.get_ref())
            .await
    }

    pub async fn unset_label(self, pool: web::Data<PgPool>, label_id: i32) -> Result<PgQueryResult, Error>{
        query("DELETE FROM notes_labels WHERE node_id = ?, label_id = ?")
            .bind(self.id)
            .bind(label_id)
            .execute(pool.get_ref())
            .await
    }

    pub async fn get_labels(self, pool: web::Data<PgPool>) -> Result<Vec<Label>, Error>{
        query(r#"SELECT l.id, l.name FROM labels l INNER JOIN notes_labels nl ON l.id = nl.label_id AND nl.note_id = $1"#)
            .bind(self.id)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get_label(self, pool: web::Data<PgPool>, label_id: i32) -> Result<Label, Error>{
        query(r#"SELECT id, name FROM labels WHERE id = ?"#)
            .bind(label_id)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn set_category(self, pool: web::Data<PgPool>, category_id: i32) -> Result<PgQueryResult, Error>{
        query("INSERT INTO notes_categories (note_id, category_id) VALUES (?, ?);")
            .bind(self.id)
            .bind(category_id)
            .execute(pool.get_ref())
            .await
    }

    pub async fn unset_category(self, pool: web::Data<PgPool>, category_id: i32) -> Result<PgQueryResult, Error>{
        query("DELETE FROM notes_categories WHERE node_id = ?, category_id = ?")
            .bind(self.id)
            .bind(category_id)
            .execute(pool.get_ref())
            .await
    }

    pub async fn get_categories(self, pool: web::Data<PgPool>) -> Result<Vec<Category>, Error>{
        query(r#"SELECT c.id, c.name FROM categories l INNER JOIN notes_categories nc ON c.id = nc.category_id AND nc.category_id = ?"#)
            .bind(self.id)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get_category(self, pool: web::Data<PgPool>, category_id: i32) -> Result<Category, Error>{
        query(r#"SELECT id, name FROM categories WHERE id = ?"#)
            .bind(category_id)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}
