use actix_web::web;
use sqlx::{query, FromRow, Error, Row, postgres::{PgPool, PgRow}};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserLabel{
    pub id: i32,
    pub user_id: i32,
    pub label_id: i32,
}

impl UserLabel{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<UserLabel>, Error>{
        query(r#"SELECT id, user_id, label_id FROM users_labels"#)
            .map(|row: PgRow| UserLabel{
                id: row.get("id"),
                user_id: row.get("user_id"),
                label_id: row.get("label_id"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<UserLabel, Error>{
        query(r#"SELECT id, user_id, label_id FROM users_labels WHERE id = ?"#)
            .bind(id)
            .map(|row: PgRow| UserLabel{
                id: row.get("id"),
                user_id: row.get("user_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn new(pool: &web::Data<PgPool>, user_id: i32, label_id: i32) -> Result<UserLabel, Error>{
        query(r#"INSERT INTO users_labels (user_id, label_id) VALUES ($1, $2) RETURNING id, user_id, label_id;"#)
            .bind(user_id)
            .bind(label_id)
            .map(|row: PgRow| UserLabel{
                id: row.get("id"),
                user_id: row.get("user_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, user_id: i32, label_id: i32) -> Result<UserLabel, Error>{
        query(r#"DELETE FROM users_labels WHERE user_id = $1 AND label_id = $2 RETURNING id, user_id, label_id;"#)
            .bind(user_id)
            .bind(label_id)
            .map(|row: PgRow| UserLabel{
                id: row.get("id"),
                user_id: row.get("user_id"),
                label_id: row.get("label_id"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}

