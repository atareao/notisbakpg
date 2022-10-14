use actix_web::{get, post, put, delete, web,
                error::{ErrorNotFound, ErrorBadRequest}, Error, HttpResponse,
                HttpRequest, http::StatusCode, test::{self, TestRequest}, App};
use anyhow::Result;
use sqlx::PgPool;
use crate::model::{note::{Note, NewNote}, category::Category,
    note_label::NoteLabel, note_category::NoteCategory, label::{Label, 
        NewLabel}};
use serde_json::Value;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

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

#[utoipa::path(
    request_body = NewNote,
    responses(
        (status = 201, description = "Note created successfully", body = Note),
    ),
    tag = "notes"
)]
#[post("/notes")]
pub async fn create_note(pool: web::Data<PgPool>, note: web::Json<NewNote>) -> Result<HttpResponse, Error>{
    Note::new(pool, note.into_inner())
       .await
       .map(|note| HttpResponse::Created().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 200, description = "The note for this id", body = Note),
        (status = 404, description = "Note not found"),
    ),
    tag = "notes"
)]
#[get("/notes/{id}")]
pub async fn read_note(req: HttpRequest, pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Note::get(pool, id)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    responses(
        (status = 200, description = "List all notes", body = [Note])
    ),
    tag = "notes"
)]
#[get("/notes")]
pub async fn read_notes(req: HttpRequest, pool: web::Data<PgPool>)->Result<HttpResponse, Error>{
    Note::all(pool)
        .await
        .map(|some_notes| HttpResponse::Ok().json(some_notes))
        .map_err(|_| ErrorBadRequest("Not found"))
}


#[utoipa::path(
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 200, description = "List all labels por a note", body = [Category])
    ),
    tag = "notes"
)]
#[get("/notes/{id}/categories/")]
pub async fn read_categories_for_note(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Category::get_categories_for_note(pool, id)
       .await
       .map(|categories| HttpResponse::Ok().json(categories))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 200, description = "List all labels por a note", body = [Label])
    ),
    tag = "notes"
)]
#[get("/notes/{id}/labels/")]
pub async fn read_labels_for_note(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Label::get_labels_for_note(pool, id)
       .await
       .map(|labels| HttpResponse::Ok().json(labels))
       .map_err(|_| ErrorNotFound("Not found"))
}


#[utoipa::path(
    request_body = Note,
    responses(
        (status = 201, description = "Note updated successfully", body = Note),
        (status = 404, description = "Note not found", body = Note),
    ),
    tag = "notes",
)]
#[put("/notes")]
pub async fn update_note(pool: web::Data<PgPool>, post: String) -> Result<HttpResponse, Error>{
    let content: Value = serde_json::from_str(&post).unwrap();
    Note::update(pool, content)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 201, description = "Deleted note", body = Note),
    ),
    tag = "notes",
)]
#[delete("/notes/{id}")]
pub async fn delete_note(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Note::delete(pool, id)
       .await
       .map(|note| HttpResponse::Ok().json(note))
       .map_err(|_| ErrorNotFound("Not found"))
}


#[utoipa::path(
    params(
        ("note_id", description = "The id of the note"),
        ("label_id", description = "The id of the label"),
    ),
    responses(
        (status = 201, description = "Add label note", body = NoteLabel),
        (status = 400, description = "Bad request", body = NoteLabel),
    ),
    tag = "notes",
)]
#[put("/notes/{note_id}/labels/{label_id}")]
pub async fn add_label_to_note(pool: web::Data<PgPool>, path: web::Path<(i32, i32)>)->Result<HttpResponse, Error>{
    let note_id = path.0;
    let label_id = path.1;
    NoteLabel::new(pool, note_id, label_id)
        .await
        .map(|note_label| HttpResponse::Ok().json(note_label))
        .map_err(|_| ErrorBadRequest("Bad Request"))
}

/// Remove a label from note by ids
///
/// Deassigned a label from a note
#[utoipa::path(
    params(
        ("note_id", description = "The id of the note"),
        ("label_id", description = "The id of the label"),
    ),
    responses(
        (status = 201, description = "Add label note", body = NoteLabel),
        (status = 400, description = "Bad request", body = NoteLabel),
    ),
    tag = "notes",
)]

#[delete("/notes/{note_id}/labels/{label_id}")]
pub async fn delete_label_from_note(pool: web::Data<PgPool>, path: web::Path<(i32, i32)>)->Result<HttpResponse, Error>{
    let note_id = path.0;
    let label_id = path.1;
    NoteLabel::delete(pool, note_id, label_id)
        .await
        .map(|note_label| HttpResponse::Ok().json(note_label))
        .map_err(|_| ErrorBadRequest("Bad Request"))
}

#[utoipa::path(
    params(
        ("note_id", description = "The id of the note"),
        ("category_id", description = "The id of the category"),
    ),
    responses(
        (status = 201, description = "Add category note", body = NoteCategory),
        (status = 400, description = "Bad request", body = NoteLabel),
    ),
    tag = "notes",
)]
#[put("/notes/{note_id}/categories/{category_id}")]
pub async fn add_category_to_note(pool: web::Data<PgPool>, path: web::Path<(i32, i32)>)->Result<HttpResponse, Error>{
    let note_id = path.0;
    let category_id = path.1;
    NoteCategory::new(pool, note_id, category_id)
        .await
        .map(|note_category| HttpResponse::Ok().json(note_category))
        .map_err(|_| ErrorBadRequest("Bad Request"))
}

/// Remove a category from note by ids
///
/// Deassigned a category from a note
#[utoipa::path(
    params(
        ("note_id", description = "The id of the note"),
        ("category_id", description = "The id of the category"),
    ),
    responses(
        (status = 201, description = "Add category to note", body = NoteCategory),
        (status = 400, description = "Bad request", body = NoteCategory),
    ),
    tag = "notes",
)]

#[delete("/notes/{note_id}/categories/{category_id}")]
pub async fn delete_category_from_note(pool: web::Data<PgPool>, path: web::Path<(i32, i32)>)->Result<HttpResponse, Error>{
    let note_id = path.0;
    let category_id = path.1;
    NoteCategory::delete(pool, note_id, category_id)
        .await
        .map(|note_category| HttpResponse::Ok().json(note_category))
        .map_err(|_| ErrorBadRequest("Bad Request"))
}
