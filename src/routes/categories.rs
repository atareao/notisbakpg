use actix_web::{get, post, put, delete, web, error::ErrorNotFound, Error,
                HttpResponse};
use anyhow::Result;
use sqlx::PgPool;
use crate::model::category::{Category, NewCategory};

#[utoipa::path(
    request_body = NewCategory,
    responses(
        (status = 201, description = "Category created successfully", body = Category),
    ),
    tag = "categories",
)]
#[post("/categories")]
pub async fn create_category(pool: web::Data<PgPool>, category: web::Json<NewCategory>) -> Result<HttpResponse, Error>{
    let name = category.into_inner().name;
    Category::new(pool, &name)
       .await
       .map(|category| HttpResponse::Created().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}

#[utoipa::path(
    params(
        ("id", description = "The id of the category"),
    ),
    responses(
        (status = 200, description = "The category fot this id", body = Category),
        (status = 404, description = "Category not found", body = Category),
    ),
    tag = "categories",
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
    ),
    tag = "categories",
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
    ),
    tag = "categories",
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
    ),
    tag = "categories",
)]
#[delete("/categories/{id}")]
pub async fn delete_category(pool: web::Data<PgPool>, path: web::Path<i32>)->Result<HttpResponse, Error>{
    let id = path.into_inner();
    Category::delete(pool, id)
       .await
       .map(|category| HttpResponse::Ok().json(category))
       .map_err(|_| ErrorNotFound("Not found"))
}
