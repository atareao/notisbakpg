use actix_web::web;
use sqlx::{query, query_as, FromRow, Error, postgres::PgPool};
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
        let categories = query_as!(Category, r#"SELECT id, name FROM categories"#)
            .fetch_all(pool.get_ref())
            .await?;
        Ok(categories)
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<Category, Error>{
        let category = query_as!(Category, r#"SELECT id, name FROM categories WHERE id=$1"#, id)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(category)
    }

    pub async fn get_last_inserted(pool: web::Data<PgPool>) -> Result<Category, Error>{
        let category = query_as!(Category, r#"SELECT id, name FROM categories WHERE id=(SELECT CURRVAL(PG_GET_SERIAL_SEQUENCE('categories', 'id')))"#)
            .fetch_one(pool.get_ref())
            .await?;
        Ok(category)
    }

    pub async fn new(pool: web::Data<PgPool>, name: &str) -> Result<Category, Error>{
        let id = query("INSERT INTO categories (name) VALUES (?);")
            .bind(name)
            .execute(pool.get_ref())
            .await?;
        Self::get_last_inserted(pool).await
    }

    pub async fn update(pool: web::Data<PgPool>, category: Category) -> Result<Category, Error>{
        query("UPDATE categories SET name=? WHERE id=?;")
            .bind(category.name)
            .bind(category.id)
            .execute(pool.get_ref())
            .await?;
        Self::get(pool, category.id).await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<String, Error>{
        query("DELETE FROM categories WHERE id = ?;")
            .bind(id)
            .execute(pool.get_ref())
            .await;
        Ok("Category deleted".to_string())
    }
}
