use actix_web::web;
use sqlx::{query_as, FromRow, Error, PgPool};
use serde::{Serialize, Deserialize};

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Label{
    pub id: i32,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NewLabel{
    pub name: String,
}

impl Label{
    pub async fn all(pool: web::Data<PgPool>) -> Result<Vec<Label>, Error>{
        query_as!(Label,
                  r#"SELECT id, name FROM labels"#)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn get(pool: web::Data<PgPool>, id: i32) -> Result<Label, Error>{
        query_as!(Label,
                  r#"SELECT id, name FROM labels WHERE id=$1"#,
                  id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn get_labels_for_note(pool: web::Data<PgPool>, note_id: i32) -> Result<Vec<Label>, Error>{
        query_as!(Label,
                  r#"SELECT l.id, l.name FROM labels l INNER JOIN notes_labels nl ON nl.label_id = l.id AND note_id=$1"#,
                  note_id)
            .fetch_all(pool.get_ref())
            .await
    }

    pub async fn new(pool: web::Data<PgPool>, name: &str) -> Result<Label, Error>{
        query_as!(Label,
                  r#"INSERT INTO labels (name) VALUES ($1) RETURNING id, name;"#,
                  name)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn update(pool: web::Data<PgPool>, label: Label) -> Result<Label, Error>{
        query_as!(Label,
                  r#"UPDATE labels SET name = $1 WHERE id = $2 RETURNING id, name;"#,
                  label.name,
                  label.id)
            .fetch_one(pool.get_ref())
            .await
    }

    pub async fn delete(pool: web::Data<PgPool>, id: i32) -> Result<Label, Error>{
        query_as!(Label,
                  r#"DELETE FROM labels WHERE id = $1 RETURNING id, name;"#,
                  id)
            .fetch_one(pool.get_ref())
            .await
    }
}

