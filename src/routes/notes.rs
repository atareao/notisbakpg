use actix_web::{get, post, put, delete, web, error::{ErrorNotFound,
    ErrorConflict, ErrorUnauthorized, ErrorBadRequest}, Error, HttpResponse};
use actix_web::{App, test};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Result;
use sqlx::PgPool;
use crate::model::{note::{Note, NewNote}, category::Category,
    note_label::NoteLabel, note_category::NoteCategory, label::Label, claims::Claims};
use serde_json::Value;

#[get("/v1/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::Ok().body("Hello world, Rust!"))
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
    context_path = "/api",
    request_body = NewNote,
    responses(
        (status = 201, description = "Created successfully", body = NewNote),
        (status = 401, description = "Error: Conflict"),
        (status = 409, description = "Error: Unauthorized")
    ),
    tag = "notes"
)]
#[post("/v1/notes")]
pub async fn create_note(pool: web::Data<PgPool>, note: web::Json<NewNote>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            Note::new(pool, note.into_inner(), user_id)
               .await
               .map(|item| HttpResponse::Created().json(item))
               .map_err(|e| ErrorNotFound("Not found"))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 200, description = "Get One", body = Note),
        (status = 404, description = "Error: Not found"),
        (status = 409, description = "Error: Unauthorized")
    ),
    tag = "notes"
)]
#[get("/v1/notes/{id}")]
pub async fn read_note(pool: web::Data<PgPool>, path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let id = path.into_inner();
            Note::get(pool, id, user_id)
               .await
               .map(|item| HttpResponse::Ok().json(item))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all", body = [Note]),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "notes"
)]
#[get("/v1/notes")]
pub async fn read_notes(pool: web::Data<PgPool>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            Note::all(pool, user_id)
                .await
                .map(|some_notes| HttpResponse::Ok().json(some_notes))
                .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}


#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 200, description = "All categories for note", body = [Category]),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "notes"
)]
#[get("/v1/notes/{id}/categories/")]
pub async fn read_categories_for_note(pool: web::Data<PgPool>,
        path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let id = path.into_inner();
            Category::get_categories_for_note(pool, id, user_id)
               .await
               .map(|items| HttpResponse::Ok().json(items))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 200, description = "All labels for ntoe", body = [Label]),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "notes"
)]
#[get("/v1/notes/{id}/labels/")]
pub async fn read_labels_for_note(pool: web::Data<PgPool>,
        path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let id = path.into_inner();
            Label::get_labels_for_note(pool, id, user_id)
               .await
               .map(|labels| HttpResponse::Ok().json(labels))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}


#[utoipa::path(
    context_path = "/api",
    request_body = Note,
    responses(
        (status = 200, description = "Updated successfully", body = Note),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "notes",
)]
#[put("/v1/notes")]
pub async fn update_note(pool: web::Data<PgPool>, post: String, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let content: Value = serde_json::from_str(&post).unwrap();
            Note::update(pool, content, user_id)
               .await
               .map(|note| HttpResponse::Ok().json(note))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the note"),
    ),
    responses(
        (status = 200, description = "Deleted successfully", body = Label),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "notes",
)]
#[delete("/v1/notes/{id}")]
pub async fn delete_note(pool: web::Data<PgPool>,
        path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let id = path.into_inner();
            Note::delete(pool, id, user_id)
               .await
               .map(|note| HttpResponse::Ok().json(note))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}


#[utoipa::path(
    context_path = "/api",
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
#[put("/v1/notes/{note_id}/labels/{label_id}")]
pub async fn add_label_to_note(pool: web::Data<PgPool>,
        path: web::Path<(i32, i32)>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(_user_id) => {
            let note_id = path.0;
            let label_id = path.1;
            NoteLabel::new(pool, note_id, label_id)
                .await
                .map(|note_label| HttpResponse::Ok().json(note_label))
                .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

/// Remove a label from note by ids
///
/// Deassigned a label from a note
#[utoipa::path(
    context_path = "/api",
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

#[delete("/v1/notes/{note_id}/labels/{label_id}")]
pub async fn delete_label_from_note(pool: web::Data<PgPool>,
        path: web::Path<(i32, i32)>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    let note_id = path.0;
    let label_id = path.1;
    NoteLabel::delete(pool, note_id, label_id)
        .await
        .map(|note_label| HttpResponse::Ok().json(note_label))
        .map_err(|e| ErrorBadRequest(e))
}

#[utoipa::path(
    context_path = "/api",
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
#[put("/v1/notes/{note_id}/categories/{category_id}")]
pub async fn add_category_to_note(pool: web::Data<PgPool>,
        path: web::Path<(i32, i32)>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    let note_id = path.0;
    let category_id = path.1;
    let user_id = Claims::get_index(credentials).unwrap();
    NoteCategory::new(pool, note_id, category_id)
        .await
        .map(|note_category| HttpResponse::Ok().json(note_category))
        .map_err(|_| ErrorBadRequest("Bad Request"))
}

/// Remove a category from note by ids
///
/// Deassigned a category from a note
#[utoipa::path(
    context_path = "/api",
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

#[delete("/v1/notes/{note_id}/categories/{category_id}")]
pub async fn delete_category_from_note(pool: web::Data<PgPool>,
        path: web::Path<(i32, i32)>)->Result<HttpResponse, Error>{
    let note_id = path.0;
    let category_id = path.1;
    NoteCategory::delete(pool, note_id, category_id)
        .await
        .map(|note_category| HttpResponse::Ok().json(note_category))
        .map_err(|_| ErrorBadRequest("Bad Request"))
}
