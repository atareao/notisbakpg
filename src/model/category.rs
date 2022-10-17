use actix_web::web;
use sqlx::{query, FromRow, Error, Row, postgres::{PgPool, PgRow}};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Category{
    #[schema(example = 1)]
    pub id: i32,
    #[schema(example = "categoria 1")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryWU{
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}
#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct NewCategory{
    #[schema(example = "categoría 1")]
    pub name: String,
}

impl Category{
    pub async fn all(pool: web::Data<PgPool>, user_id: i32) -> Result<Vec<Category>, Error>{
        let sql = r#"SELECT id, name
        FROM categories
        WHERE user_id = $1
        "#;
        query(sql)
            .bind(user_id)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32, user_id: i32) -> Result<Category, Error>{
        let sql = r#"SELECT id, name
        FROM categories
        WHERE id = $1 AND user_id = $2
        "#;
        query(sql)
            .bind(id)
            .bind(user_id)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, name: &str, user_id: i32) -> Result<Category, Error>{
        query(r#"INSERT INTO categories (name, user_id) VALUES ($1, $2) RETURNING id, name;"#)
            .bind(name)
            .bind(user_id)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, category: Category) -> Result<Category, Error>{
        query(r#"UPDATE categories SET name=$2 WHERE id=$1 RETURNING id, name;"#)
            .bind(category.id)
            .bind(category.name)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<Category, Error>{
        query(r#"DELETE FROM categories WHERE id = $1 RETURNING id, name;"#)
            .bind(id)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_categories_for_note(pool: web::Data<PgPool>, note_id: i32) -> Result<Vec<Category>, Error>{
        query(r#"SELECT c.id, c.name FROM categories c INNER JOIN notes_categories nc ON nc.category_id = c.id AND note_id = $1"#)
            .bind(note_id)
            .map(|row: PgRow| Category{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool.get_ref())
            .await
    }
}
