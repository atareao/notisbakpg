use actix_web::{get, post, put, delete, web, error::ErrorNotFound, Error,
                HttpResponse};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use anyhow::Result;
use sqlx::PgPool;
use crate::model::{category::{Category, NewCategory}, claims::Claims};

#[utoipa::path(
    context_path = "/api",
    request_body = NewCategory,
    responses(
        (status = 201, description = "Category created successfully", body = Category),
    ),
    tag = "categories",
)]
#[post("/v1/categories")]
pub async fn create_category(pool: web::Data<PgPool>, category: web::Json<NewCategory>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    let user_id = Claims::get_index(credentials).unwrap();
    let name = category.into_inner().name;
    Category::new(pool, &name, user_id)
       .await
       .map(|category| HttpResponse::Created().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the category"),
    ),
    responses(
        (status = 200, description = "The category fot this id", body = Category),
        (status = 404, description = "Category not found", body = Category),
    ),
    tag = "categories",
)]
#[get("/v1/categories/{id}")]
pub async fn read_category(pool: web::Data<PgPool>, path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    let user_id = Claims::get_index(credentials).unwrap();
    let id = path.into_inner();
    Category::get(pool, id, user_id)
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all categories", body = [Category])
    ),
    tag = "categories",
)]
#[get("/v1/categories")]
pub async fn read_categories(pool: web::Data<PgPool>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    let user_id = Claims::get_index(credentials).unwrap();
    Category::all(pool, user_id)
       .await
       .map(|some_categories| HttpResponse::Ok().json(some_categories))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    request_body = Category,
    responses(
        (status = 201, description = "Category updated successfully", body = Category),
        (status = 404, description = "Category not found", body = Category),
    ),
    tag = "categories",
)]
#[put("/v1/categories")]
pub async fn update_category(pool: web::Data<PgPool>, category: web::Json<Category>) -> Result<HttpResponse, Error>{
    Category::update(pool, category.into_inner())
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the category"),
    ),
    responses(
        (status = 201, description = "Category deleted successfully", body = Category),
    ),
    tag = "categories",
)]
#[delete("/v1/categories/{id}")]
pub async fn delete_category(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Category::delete(pool, id)
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}
