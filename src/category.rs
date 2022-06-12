use actix_web::web;
use sqlx::{query_as, FromRow, Error, PgPool};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Category{
    pub id: i32,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewCategory{
    pub name: String,
}

impl Category{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<Category>, Error>{
        query_as!(Category, r#"SELECT id, name FROM categories"#)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<Category, Error>{
        query_as!(Category, r#"SELECT id, name FROM categories WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, name: &str) -> Result<Category, Error>{
        query_as!(Category,
                  r#"INSERT INTO categories (name) VALUES ($1) RETURNING id, name;"#,
                  name)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, category: Category) -> Result<Category, Error>{
        query_as!(Category,
                  r#"UPDATE categories SET name=$2 WHERE id=$1 RETURNING id, name;"#,
                  category.id,
                  category.name)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<Category, Error>{
        query_as!(Category,
                  r#"DELETE FROM categories WHERE id = $1 RETURNING id, name;"#,
                  id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_categories_for_note(pool: web::Data<PgPool>, note_id: i32) -> Result<Vec<Category>, Error>{
        query_as!(Category,
                  r#"SELECT c.id, c.name FROM categories c INNER JOIN notes_categories nc ON nc.category_id = c.id AND note_id = $1"#,
                  note_id)
            .fetch_all(pool.get_ref())
            .await
    }
}
