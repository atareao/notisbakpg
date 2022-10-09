use actix_web::web;
use chrono::{NaiveDateTime, Utc};
use sqlx::{query, FromRow, Error, Row, postgres::{PgPool, PgRow}};
use serde::{Serialize, Deserialize};

//https://github.com/juhaku/utoipa

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User{
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials{
    pub email: String,
    pub password: String,
}


impl User{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<User>, Error>{
        query(r#"SELECT id, email, password, created_at, updated_at FROM users"#)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<User, Error>{
        query(r#"SELECT id, email, password, created_at, updated_at FROM users WHERE id = $1"#)
            .bind(id)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("title"),
                password: row.get("body"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_by_email(pool: web::Data<PgPool>, email: &str) -> Result<User, Error>{
        query(r#"SELECT id, email, password, created_at, updated_at FROM users WHERE email = $1"#)
            .bind(email)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("title"),
                password: row.get("body"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, credentials: Credentials) -> Result<User, Error>{
        let email = credentials.email;
        let password = format!("{:x}", md5::compute(credentials.password));
        let created_at = Utc::now().naive_utc();
        let updated_at = Utc::now().naive_utc();
        query(r#"INSERT INTO users (email, password, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING id, email, password, created_at, updated_at;"#,)
            .bind(email)
            .bind(password)
            .bind(created_at)
            .bind(updated_at)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}
