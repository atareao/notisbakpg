use actix_web::{get, post, put, delete, web,
                error::{ErrorNotFound, ErrorBadRequest}, Error, HttpResponse,
                HttpRequest, http::StatusCode, test::{self, TestRequest}, App};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Result;
use sqlx::PgPool;
use crate::{label::{Label, NewLabel}, utils::test};
use serde_json::Value;

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all labels", body = [Label])
    ),
    tag  = "labels"
)]
#[get("/v1/labels")]
pub async fn read_labels(pool: web::Data<PgPool>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    eprintln!("{}", credentials.token());
    test(credentials);
    Label::all(pool)
       .await
       .map(|some_labels| HttpResponse::Ok().json(some_labels))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    request_body = NewLabel,
    responses(
        (status = 201, description = "Label created successfully", body = NewLabel),
    ),
    tag  = "labels"
)]
#[post("/v1/labels")]
pub async fn create_label(pool: web::Data<PgPool>, body: String) -> Result<HttpResponse, Error>{
    let content: Value = serde_json::from_str(&body).unwrap();
    let name = content.get("name").as_ref().unwrap().as_str().unwrap();
    Label::new(pool, name)
       .await
       .map(|label| HttpResponse::Ok().json(label))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the label"),
    ),
    responses(
        (status = 201, description = "Label created successfully", body = Label),
    ),
    tag  = "labels"
)]
#[get("/v1/labels/{id}")]
pub async fn read_label(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Label::get(pool, id)
       .await
       .map(|label| HttpResponse::Ok().json(label))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    request_body = Label,
    responses(
        (status = 201, description = "Label updated successfully", body = Label),
    ),
    tag  = "labels"
)]
#[put("/v1/labels")]
pub async fn update_label(pool: web::Data<PgPool>, label: web::Json<Label>) -> Result<HttpResponse, Error>{
    Label::update(pool, label.into_inner())
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the label"),
    ),
    responses(
        (status = 201, description = "Label created successfully", body = Label),
    ),
    tag  = "labels"
)]
#[delete("/v1/labels/{id}")]
pub async fn delete_label(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Label::delete(pool, id)
       .await
       .map(|label| HttpResponse::Ok().json(label))
       .map_err(|_| ErrorNotFound("Not found"))
}

