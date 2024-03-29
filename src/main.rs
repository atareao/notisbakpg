mod routes;
mod model;

use sqlx::{postgres::PgPoolOptions, migrate::{Migrator, MigrateDatabase}};
use actix_web::{App, HttpServer, web::{self, Data}, dev::ServiceRequest,
    middleware::Logger, Error};
use dotenv::dotenv;
use utoipa::{OpenApi, Modify, openapi};
use utoipa_swagger_ui::{SwaggerUi, Url};
use std::{env, path::Path};
use env_logger::Env;
use actix_web_httpauth::{extractors::bearer::BearerAuth,
    middleware::HttpAuthentication};
use jsonwebtoken::{decode, decode_header, Validation, DecodingKey};
use openapi::security::{SecurityScheme, HttpBuilder, HttpAuthScheme};


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


    //    #[openapi(modifiers(&SecurityAddon))]

    #[derive(OpenApi)]
    #[openapi(
        paths(
            routes::labels::create_label,
            routes::labels::read_label,
            routes::labels::read_labels,
            routes::labels::update_label,
            routes::labels::delete_label,
            routes::categories::create_category,
            routes::categories::read_category,
            routes::categories::read_categories,
            routes::categories::update_category,
            routes::categories::delete_category,
            routes::notes::create_note,
            routes::notes::read_note,
            routes::notes::read_notes,
            routes::notes::read_labels_for_note,
            routes::notes::read_categories_for_note,
            routes::notes::update_note,
            routes::notes::delete_note,
            routes::notes::add_label_to_note,
            routes::notes::add_category_to_note,
            routes::notes::delete_label_from_note,
            routes::notes::delete_category_from_note,
        ),
        components(
            schemas(model::label::Label,
                    model::label::NewLabel,
                    model::category::Category,
                    model::category::NewCategory,
                    model::note::Note,
                    model::note::NewNote,
                    model::note::UpdateNote)
        ),
    )]
    struct ApiDoc;

    struct SecurityAddon;

    //impl Modify for SecurityAddon{
    //    fn modify(&self, openapi: & mut utoipa::openapi::OpenApi){
    //        openapi.components = Some(
    //            utoipa::openapi::ComponentsBuilder::new()
    //                .security_scheme(
    //                    "api_jwt_token",
    //                    SecurityScheme::Http(
    //                        HttpBuilder::new()
    //                            .scheme(HttpAuthScheme::Bearer)
    //                            .bearer_format("JWT")
    //                            .build(),
    //                    ),
    //                )
    //                .build(),
    //        )
    //    }
    //}

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

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move ||{
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(web::scope("test")
                .wrap(auth.clone())
                .service(routes::notes::root)
            )
            .service(web::scope("auth")
                .service(routes::users::login)
                .service(routes::users::register)
                .service(web::scope("")
                    .wrap(auth.clone())
                    .service(routes::users::validate)
                )
            )
            .service(
                web::scope("api")
                .wrap(auth.clone())
                .service(routes::notes::root)
                .service(routes::notes::create_note)
                .service(routes::notes::read_note)
                .service(routes::notes::read_notes)
                .service(routes::notes::read_labels_for_note)
                .service(routes::notes::read_categories_for_note)
                .service(routes::notes::update_note)
                .service(routes::notes::delete_note)
                .service(routes::notes::add_label_to_note)
                .service(routes::notes::add_category_to_note)
                .service(routes::notes::delete_label_from_note)
                .service(routes::notes::delete_category_from_note)
                .service(routes::categories::create_category)
                .service(routes::categories::read_category)
                .service(routes::categories::read_categories)
                .service(routes::categories::update_category)
                .service(routes::categories::delete_category)
                .service(routes::labels::create_label)
                .service(routes::labels::read_label)
                .service(routes::labels::read_labels)
                .service(routes::labels::update_label)
                .service(routes::labels::delete_label)
                .service(routes::users::login)
                .service(routes::users::register)
            )
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .urls(vec![
                        (Url::new("api1", "/api-doc/openapi1.json"),
                        ApiDoc::openapi()),
                    ])
            )
    })
    .workers(2)
    .bind(format!("0.0.0.0:{}", &port))
    .unwrap()
    .run()
    .await
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)>{
    eprint!("{}", req.path());
    eprint!("{:?}", req);
    println!("Estoy aqui");
    eprintln!("{}", credentials.token());
    let decoded = decode::<routes::users::Claims>(credentials.token(),
        &DecodingKey::from_secret("SECRETO".as_ref()),
        &Validation::default());
    let header = decode_header(credentials.token()).unwrap();
    let jwt = header.jwk;
    eprintln!("BearerAuth {:?}", credentials);
    eprintln!("BearerAuth {:?}", decoded);
    eprintln!("decoded {:?}", jwt);
    Ok(req)
}
