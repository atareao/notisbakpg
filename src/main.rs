mod note;
mod label;
mod category;
mod note_label;
mod note_category;
mod routes;

use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use sqlx::{postgres::PgPoolOptions, migrate::{Migrator, MigrateDatabase}};
use actix_web::{App, HttpServer, web::Data, dev::ServiceRequest};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::{SwaggerUi, Url};
use std::{env, path::Path};
use routes::{root,
             all_notes, create_note, read_note, update_note, delete_note,
             all_categories, new_category, all_labels, new_label, read_label};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let port = env::var("PORT").expect("PORT not set");

    if !sqlx::Postgres::database_exists(&db_url).await.unwrap(){
        sqlx::Postgres::create_database(&db_url).await.unwrap()
    }

    // Migrate the database
    let migrations = if env::var("RUST_ENV") == Ok("production".to_string()) {
        // Productions migrations dir
        std::env::current_exe()?.parent().unwrap().join("migrations")
    } else {
        // Development migrations dir
        let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&crate_dir)
            .join("./migrations")
    };
    println!("{}", &migrations.display());


    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::all_labels,
            routes::new_label,
        ),
        components(
            schemas(label::Label)
        ),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("pool failed");

    // Do migration
    Migrator::new(migrations)
        .await.unwrap()
        .run(&pool)
        .await.unwrap();

    HttpServer::new(move ||{
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(root)
            .service(all_notes)
            .service(create_note)
            .service(read_note)
            .service(update_note)
            .service(delete_note)
            .service(all_categories)
            .service(new_category)
            .service(all_labels)
            .service(new_label)
            .service(read_label)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").urls(vec![
                (Url::new("api1", "/api-doc/openapi1.json"),
                 ApiDoc::openapi()),
            ]))
    })
    .bind(format!("0.0.0.0:{}", &port))
    .unwrap()
    .run()
    .await
}

async fn basic_auth_validator(req: ServiceRequest, credentials: BasicAuth)->Result<ServiceRequest, std::io::Error>{
    let config = req
        .app_data::<Config>()
        .map(|data| data.to_owned())
        .unwrap_or_else(Default::default);
    if let Ok(res) = validate_credentials(credentials.user_id(), credentials.password().unwrap().trim()){
        if res {
            return Ok(req);
        }
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"))
}

fn validate_credentials(user_id: &str, user_password: &str) -> Result<bool, std::io::Error>{
    if user_id.eq("karl") && user_password.eq("password") {
        return Ok(true);
    }
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Authentication failed!"))
}
