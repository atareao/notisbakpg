use actix_web::{get, post, put, delete, web,
                error::{ErrorNotFound, ErrorBadRequest}, Error, HttpResponse,
                HttpRequest, http::StatusCode, test::{self, TestRequest}, App};
use anyhow::Result;
use sqlx::PgPool;
use crate::note::{Note, NewNote};
use crate::category::{Category, NewCategory};
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

#[utoipa::path(
    request_body = NewCategory,
    responses(
        (status = 201, description = "Category created successfully", body = Category),
    )
)]
#[post("/categories")]
pub async fn create_category(pool: web::Data<PgPool>, category: web::Json<NewCategory>) -> Result<HttpResponse, Error>{
    let name = category.into_inner().name;
    Category::new(pool, &name)
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the category"),
    ),
    responses(
        (status = 200, description = "The category fot this id", body = Category),
        (status = 404, description = "Category not found", body = Category),
    )
)]
#[get("/categories/{id}")]
pub async fn read_category(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Category::get(pool, id)
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    responses(
        (status = 200, description = "List all categories", body = [Category])
    )
)]
#[get("/categories")]
pub async fn read_categories(pool: web::Data<PgPool>) -> Result<HttpResponse, Error>{
    Category::all(pool)
       .await
       .map(|some_categories| HttpResponse::Ok().json(some_categories))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    request_body = Category,
    responses(
        (status = 201, description = "Category updated successfully", body = Category),
        (status = 404, description = "Category not found", body = Category),
    )
)]
#[put("/categories")]
pub async fn update_category(pool: web::Data<PgPool>, category: web::Json<Category>) -> Result<HttpResponse, Error>{
    Category::update(pool, category.into_inner())
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the category"),
    ),
    responses(
        (status = 201, description = "Category deleted successfully", body = Category),
    )
)]
#[delete("/categories/{id}")]
pub async fn delete_category(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Category::delete(pool, id)
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}
