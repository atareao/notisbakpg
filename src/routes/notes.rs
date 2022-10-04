use actix_web::{get, post, put, delete, web,
                error::{ErrorNotFound, ErrorBadRequest}, Error, HttpResponse,
                HttpRequest, http::StatusCode, test::{self, TestRequest}, App};
use anyhow::Result;
use sqlx::PgPool;
use crate::note::{Note, NewNote};
use crate::label::{Label, NewLabel};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use bytes::Bytes;

/// Todo endpoint error responses
#[derive(Serialize, Deserialize, Clone, ToSchema, Debug)]
pub(super) enum ErrorResponse {
    /// When Todo is not found by search term.
    NotFound(String),
    /// When there is a conflict storing a new todo.
    Conflict(String),
    /// When todo enpoint was called without correct credentials
    Unauthorized(String),
}

#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hello world, Rust!"))
}

#[actix_web::test]
async fn test_index() {
    let app = test::init_service(
        App::new().service(root)
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .to_request();

    let result = test::call_and_read_body(&app, req).await;
    assert_eq!(result, bytes::Bytes::from_static(b"Hello world, Rust!"));
}

#[get("/notes")]
pub async fn all_notes(req: HttpRequest, pool: web::Data<PgPool>)->Result<HttpResponse, Error>{
    // let token = req.headers().get("token").expect("No token provided").to_str().unwrap();
    //Note::all(pool, token)
    Note::all(pool)
        .await
        .map(|some_notes| HttpResponse::Ok().json(some_notes))
        .map_err(|_| ErrorBadRequest("Not found"))
}

#[post("/notes")]
pub async fn create_note(pool: web::Data<PgPool>, data: web::Json<NewNote>) -> Result<HttpResponse, Error>{
    Note::new(pool, &data.into_inner().title, None)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[get("/notes/{note_id}/labels/")]
pub async fn read_labels_for_note(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let note_id = path.into_inner();
    Label::get_labels_for_note(pool, note_id)
       .await
       .map(|labels| HttpResponse::Ok().json(labels))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[get("/notes/{note_id}")]
pub async fn read_note(req: HttpRequest, pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let note_id = path.into_inner();
    Note::get(pool, note_id)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[put("/notes")]
pub async fn update_note(pool: web::Data<PgPool>, post: String) -> Result<HttpResponse, Error>{
    let content: Value = serde_json::from_str(&post).unwrap();
    Note::update(pool, content)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[delete("/notes/{note_id}")]
pub async fn delete_note(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let note_id = path.into_inner();
    Note::delete(pool, note_id)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}