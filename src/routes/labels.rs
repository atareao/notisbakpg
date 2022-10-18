use actix_web::{get, post, put, delete, web, error::{ErrorNotFound,
    ErrorConflict, ErrorUnauthorized}, Error, HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Result;
use sqlx::PgPool;
use crate::model::{label::Label, claims::Claims, user_label::UserLabel};
use serde_json::Value;

#[utoipa::path(
    context_path = "/api",
    request_body = NewLabel,
    responses(
        (status = 201, description = "Created successfully", body = Label),
        (status = 401, description = "Error: Conflict"),
        (status = 409, description = "Error: Unauthorized")
    ),
    tag  = "labels"
)]
#[post("/v1/labels")]
pub async fn create_label(pool: web::Data<PgPool>, body: String, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let content: Value = serde_json::from_str(&body).unwrap();
            let name = content.get("name").as_ref().unwrap().as_str().unwrap();
            Label::new(&pool, name, user_id)
                .await
                .map(|item| HttpResponse::Ok().json(item))
                .map_err(|e| ErrorConflict(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the label"),
    ),
    responses(
        (status = 200, description = "Get One", body = Label),
        (status = 404, description = "Error: Not found"),
        (status = 409, description = "Error: Unauthorized")
    ),
    tag  = "labels"
)]
#[get("/v1/labels/{id}")]
pub async fn read_label(pool: web::Data<PgPool>, path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let id = path.into_inner();
            Label::get(pool, id, user_id)
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
        (status = 200, description = "List all", body = [Label]),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag  = "labels",
)]
#[get("/v1/labels")]
pub async fn read_labels(pool: web::Data<PgPool>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            Label::all(pool, user_id)
               .await
               .map(|items| HttpResponse::Ok().json(items))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}


#[utoipa::path(
    context_path = "/api",
    request_body = Label,
    responses(
        (status = 201, description = "Updated successfully", body = Label),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag  = "labels"
)]
#[put("/v1/labels")]
pub async fn update_label(pool: web::Data<PgPool>, label: web::Json<Label>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(_user_id) => {
            Label::update(pool, label.into_inner())
               .await
               .map(|item| HttpResponse::Ok().json(item))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the label"),
    ),
    responses(
        (status = 201, description = "Deleted successfully", body = Label),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag  = "labels"
)]
#[delete("/v1/labels/{id}")]
pub async fn delete_label(pool: web::Data<PgPool>, path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(_user_id) => {
            let id = path.into_inner();
            Label::delete(pool, id)
               .await
               .map(|item| HttpResponse::Ok().json(item))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(ErrorUnauthorized(e)),
    }
}

