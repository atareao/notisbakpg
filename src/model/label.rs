use actix_web::web;
use sqlx::{query, FromRow, Error, Row, postgres::{PgPool, PgRow}};
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Label{
    #[schema(example = "1")]
    pub id: i32,
    #[schema(example = "etiqueta 1")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LabelWU{
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct NewLabel{
    #[schema(example = "etiqueta 1")]
    pub name: String,
}

impl Label{
    pub async fn all(pool: web::Data<PgPool>, user_id: i32) -> Result<Vec<Label>, Error>{
        let sql = r#"SELECT id, name
        FROM labels l
        WHERE user_id = $1
        "#;
        query(sql)
            .bind(user_id)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32, user_id: i32) -> Result<Label, Error>{
        let sql = r#"SELECT id, name
        FROM labels l
        WHERE l.id = $1 AND user_id = $2
        "#;
        query(sql)
            .bind(id)
            .bind(user_id)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_labels_for_note(pool: web::Data<PgPool>, note_id: i32, user_id: i32) -> Result<Vec<Label>, Error>{
        query(r#"SELECT l.id, l.name FROM labels l INNER JOIN notes_labels nl ON nl.label_id = l.id AND note_id=$1 AND l.user_id = $2"#)
            .bind(note_id)
            .bind(user_id)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn new(pool: &web::Data<PgPool>, name: &str, user_id: i32) -> Result<Label, Error>{
        query(r#"INSERT INTO labels (name, user_id) VALUES ($1, $2) RETURNING id, name;"#)
            .bind(name)
            .bind(user_id)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, label: Label) -> Result<Label, Error>{
        query(r#"UPDATE labels SET name = $2 WHERE id = $1 RETURNING id, name;"#)
            .bind(label.id)
            .bind(label.name)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<Label, Error>{
        query(r#"DELETE FROM labels WHERE id = $1 RETURNING id, name;"#)
            .bind(id)
            .map(|row: PgRow| Label{
                id: row.get("id"),
                name: row.get("name"),
            })
            .fetch_one(pool.get_ref())
            .await
    }
}

