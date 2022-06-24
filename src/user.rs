use actix_web::web;
use chrono::{NaiveDateTime, Utc};
use sqlx::{query, query_as, FromRow, Error, postgres::{PgPool, PgQueryResult}};
use serde::{Serialize, Deserialize};
use utoipa::Component;

//https://github.com/juhaku/utoipa

#[derive(Debug, FromRow, Serialize, Deserialize, Component)]
pub struct User{
    pub id: i32,
    pub name: String,
    pub token: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewUser{
    pub name: String,
    pub email: String,
    pub token: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize, Component)]
pub struct UpdateUser{
    pub id: i32,
    pub name: String,
}

mod note_api{
    use crate::note::Note;

    #[utoipa::path(
        get,
        path = "/users/{id}",
        responses(
            (status = 200, description = "User found succesfully", body = Note),
            (status = 404, description = "User was not found")
        ),
        params(
            ("id" = i32, path, description = "User database id to get User for"),
        )
    )]
    async fn get_user_by_id(user_id: i32) -> Note {
        let current = chrono::Utc::now().naive_utc();
        User {
            id: user_id,
            name: "Sample name".to_string(),
            email: "Sample email".to_string(),
            token: "Sample token".to_string(),
            created_at: current,
            updated_at: current,
        }
    }
}

impl User{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<User>, Error>{
        query_as!(Note, r#"SELECT id, name, email, token, created_at, updated_at, user_id FROM users"#)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<User, Error>{
        query_as!(Note, r#"SELECT id, name, email, token, created_at, updated_at FROM users WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, title: &str, body_option: Option<&str>) -> Result<Note, Error>{
        let body = body_option.unwrap_or("");
        let created_at = Utc::now().naive_utc();
        let updated_at = Utc::now().naive_utc();
        query_as!(Note,
                  r#"INSERT INTO notes (title, body, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING id, title, body, created_at, updated_at;"#,
                  title,
                  body,
                  created_at,
                  updated_at)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, note: UpdateNote) -> Result<Note, Error>{
        let updated_at = Utc::now().naive_utc();
        query_as!(Note,
                  r#"UPDATE notes SET title=$1, body=$2, updated_at=$3 WHERE id=$4 RETURNING id, title, body, created_at, updated_at;"#,
                  note.title,
                  note.body,
                  updated_at,
                  note.id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<Note, Error>{
        query_as!(Note,
                  r#"DELETE FROM notes WHERE id = $1 RETURNING id, title, body, created_at, updated_at;"#,
                  id)
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
        query_as!(Label, r#"SELECT l.id, l.name FROM labels l INNER JOIN notes_labels nl ON l.id = nl.label_id AND nl.note_id = $1"#, self.id)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get_label(self, pool: web::Data<PgPool>, label_id: i32) -> Result<Label, Error>{
        query_as!(Label, r#"SELECT id, name FROM labels WHERE id = $1"#, label_id)
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
        query_as!(Category, r#"SELECT c.id, c.name FROM categories c INNER JOIN notes_categories nc ON c.id = nc.category_id AND nc.note_id = $1"#, self.id)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get_category(self, pool: web::Data<PgPool>, category_id: i32) -> Result<Category, Error>{
        query_as!(Category, r#"SELECT id, name FROM categories WHERE id = $1"#, category_id)
            .fetch_one(pool.get_ref())
            .await
    }
}
