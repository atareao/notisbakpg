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
        (status = 201, description = "Created successfully", body = Category),
        (status = 403, description = "Error: Unauthorized"),
        (status = 409, description = "Error: Conflict"),
    ),
    tag = "categories",
)]
#[post("/v1/categories")]
pub async fn create_category(pool: web::Data<PgPool>, category: web::Json<NewCategory>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let name = category.into_inner().name;
            Category::new(pool, &name, user_id)
                .await
                .map(|category| HttpResponse::Ok().json(category))
                .map_err(|e| actix_web::error::ErrorConflict(e))
        },
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the category"),
    ),
    responses(
        (status = 200, description = "Get One", body = Category),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "categories",
)]
#[get("/v1/categories/{id}")]
pub async fn read_category(pool: web::Data<PgPool>, path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, actix_web::Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            let id = path.into_inner();
            Category::get(pool, id, user_id)
                .await
                .map(|category| HttpResponse::Ok().json(category))
                .map_err(|e| actix_web::error::ErrorNotFound(e))
        },
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "List all", body = [Category]),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "categories",
)]
#[get("/v1/categories")]
pub async fn read_categories(pool: web::Data<PgPool>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(user_id) => {
            Category::all(pool, user_id)
               .await
               .map(|some_categories| HttpResponse::Ok().json(some_categories))
               .map_err(|e| actix_web::error::ErrorNotFound(e))
        },
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    request_body = Category,
    responses(
        (status = 201, description = "Updated successfully", body = Category),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "categories",
)]
#[put("/v1/categories")]
pub async fn update_category(pool: web::Data<PgPool>, category: web::Json<Category>, credentials: BearerAuth) -> Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(_user_id) => {
            Category::update(pool, category.into_inner())
               .await
               .map(|category| HttpResponse::Ok().json(category))
               .map_err(|e| actix_web::error::ErrorConflict(e))
        },
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e)),
    }
}

#[utoipa::path(
    context_path = "/api",
    params(
        ("id", description = "The id of the category"),
    ),
    responses(
        (status = 201, description = "Deleted successfully", body = Category),
        (status = 404, description = "Error: Not found"),
        (status = 403, description = "Error: Unauthorized")
    ),
    tag = "categories",
)]
#[delete("/v1/categories/{id}")]
pub async fn delete_category(pool: web::Data<PgPool>, path: web::Path<i32>, credentials: BearerAuth)->Result<HttpResponse, Error>{
    match Claims::get_index(credentials) {
        Ok(_user_id) => {
            let id = path.into_inner();
            Category::delete(pool, id)
               .await
               .map(|category| HttpResponse::Ok().json(category))
               .map_err(|e| ErrorNotFound(e))
        },
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e)),
    }
}
