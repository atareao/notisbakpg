mod note;
mod label;
mod category;
mod note_label;
mod note_category;
mod routes;
mod label_api;

use actix_web_httpauth::extractors::{basic::{BasicAuth, Config}, AuthenticationError};
use sqlx::postgres::PgPoolOptions;
use actix_web::{App, HttpServer, web::Data, dev::ServiceRequest, Error};
use dotenv::dotenv;
use utoipa_swagger_ui::SwaggerUi;
use utoipa::OpenApi;
use std::env;
use label::Label;
use routes::{root,
             all_notes, create_note, read_note, update_note, delete_note,
             all_categories, new_category, all_labels, new_label, read_label};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("Database not found");

    #[derive(OpenApi)]
    #[openapi(
        handlers(
            label_api::get_label_by_id
        ),
        components(Label),
        tags(
            (name = "todo", description = "Todo management endpoints.")
        ),
    )]
    struct ApiDoc;
    //println!("{}", ApiDoc::openapi().to_pretty_json().unwrap());
    let openapi = ApiDoc::openapi();

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .expect("pool failed");

    // Do migration
    sqlx::migrate!().run(&pool).await.expect("Can not migrate");

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
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", openapi.clone()),
                )
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}

async fn basic_auth_validator(req: ServiceRequest, credentials: BasicAuth)->Result<ServiceRequest, std::io::Error>{
    let config = req
        .app_data::<Config>()
        .map(|data| data.get_ref().clone())
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
