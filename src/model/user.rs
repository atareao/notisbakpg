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
    pub login: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IUser{
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials{
    pub email: String,
    pub password: String,
}


impl User{
    pub async fn all(pool: &web::Data<PgPool>) -> Result<Vec<User>, Error>{
        query(r#"SELECT id, email, password, created_at, updated_at, login FROM users"#)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                login: row.get("login"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: &web::Data<PgPool>, id: i32) -> Result<User, Error>{
        query(r#"SELECT id, email, password, created_at, updated_at, login FROM users WHERE id = $1"#)
            .bind(id)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                login: row.get("login"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_by_email(pool: &web::Data<PgPool>, email: &str) -> Result<User, Error>{
        query(r#"SELECT id, email, password, created_at, updated_at, login FROM users WHERE email = $1"#)
            .bind(email)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                login: row.get("login"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: &web::Data<PgPool>, credentials: Credentials) -> Result<User, Error>{
        let email = credentials.email;
        let password = format!("{:x}", md5::compute(credentials.password));
        let created_at = Utc::now().naive_utc();
        let updated_at = Utc::now().naive_utc();
        let login = false;
        query(r#"INSERT INTO users (email, password, created_at, updated_at, login) VALUES ($1, $2, $3, $4, $5) RETURNING id, email, password, created_at, updated_at, login;"#,)
            .bind(email)
            .bind(password)
            .bind(created_at)
            .bind(updated_at)
            .bind(login)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                login: row.get("login"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn set_login(pool: &web::Data<PgPool>, id: i32, login: bool) -> Result<User, Error>{
        query(r#"UPDATE users set login = $1 WHERE id = $2 RETURNING id, email, password, created_at, updated_at, login;"#,)
            .bind(login)
            .bind(id)
            .map(|row: PgRow| User{
                id: row.get("id"),
                email: row.get("email"),
                password: row.get("password"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                login: row.get("login"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}
