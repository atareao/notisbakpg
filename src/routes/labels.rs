use actix_web::{get, post, put, delete, web,
                error::{ErrorNotFound, ErrorBadRequest}, Error, HttpResponse,
                HttpRequest, http::StatusCode, test::{self, TestRequest}, App};
use anyhow::Result;
use sqlx::PgPool;
use crate::label::{Label, NewLabel};
use serde_json::Value;

#[utoipa::path(
    responses(
        (status = 200, description = "List all labels", body = [Label])
    )
)]
#[get("/labels")]
pub async fn read_labels(pool: web::Data<PgPool>) -> Result<HttpResponse, Error>{
    Label::all(pool)
       .await
       .map(|some_labels| HttpResponse::Ok().json(some_labels))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    request_body = NewLabel,
    responses(
        (status = 201, description = "Label created successfully", body = NewLabel),
    )
)]
#[post("/labels")]
pub async fn create_label(pool: web::Data<PgPool>, body: String) -> Result<HttpResponse, Error>{
    let content: Value = serde_json::from_str(&body).unwrap();
    let name = content.get("name").as_ref().unwrap().as_str().unwrap();
    Label::new(pool, name)
       .await
       .map(|label| HttpResponse::Ok().json(label))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the label"),
    ),
    responses(
        (status = 201, description = "Label created successfully", body = Label),
    )
)]
#[get("/labels/{id}")]
pub async fn read_label(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Label::get(pool, id)
       .await
       .map(|label| HttpResponse::Ok().json(label))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    request_body = Label,
    responses(
        (status = 201, description = "Label updated successfully", body = Label),
    )
)]
#[put("/labels")]
pub async fn update_label(pool: web::Data<PgPool>, label: web::Json<Label>) -> Result<HttpResponse, Error>{
    Label::update(pool, label.into_inner())
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the label"),
    ),
    responses(
        (status = 201, description = "Label created successfully", body = Label),
    )
)]
#[delete("/labels/{id}")]
pub async fn delete_label(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Label::delete(pool, id)
       .await
       .map(|label| HttpResponse::Ok().json(label))
       .map_err(|_| ErrorNotFound("Not found"))
}
