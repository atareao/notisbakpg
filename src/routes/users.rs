use actix_web::{post, get, web, Error, HttpResponse, HttpRequest};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;
use jsonwebtoken::{encode, Header, EncodingKey};

use crate::user::{Credentials, User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}
impl Claims{
    pub fn new(index: i32)-> Self{
        Self{
            sub: index.to_string(),
            exp: usize::try_from(jsonwebtoken::get_current_timestamp()).unwrap() + 50000,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Response{
    code: String,
    message: String,
    token: Option<String>,
}


#[post("/login")]
pub async fn login(req: HttpRequest, pool: web::Data<PgPool>, credentials: web::Json<Credentials>) -> Result<HttpResponse, Error>{
    println!("{:?}", req);
    let user = User::get_by_email(&pool, &credentials.email).await.unwrap();
    let password = format!("{:x}", md5::compute(&credentials.password));
    if user.password == password{
        User::set_login(&pool, user.id, true).await.unwrap();
        let claims = Claims::new(user.id);
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("SECRETO".as_ref())).unwrap();
        return Ok(HttpResponse::Ok().json(Response{
            code: "Ok".to_string(),
            message: "Valid credentials".to_string(),
            token: Some(token),
        }));
    }
    Err(actix_web::error::ErrorUnauthorized("Invalid credentials".to_string()))
}

#[post("/register")]
pub async fn register(req: HttpRequest, pool: web::Data<PgPool>, credentials: web::Json<Credentials>) -> Result<HttpResponse, Error>{
    println!("{:?}", req);
    let user = User::new(&pool, credentials.into_inner())
        .await
        .unwrap();
        let claims = Claims::new(user.id);
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("SECRETO".as_ref())).unwrap();
        return Ok(HttpResponse::Created().json(Response{
            code: "Ok".to_string(),
            message: "User created".to_string(),
            token: Some(token),
        }));
}


#[get("/validate")]
pub async fn validate(req: HttpRequest, pool: web::Data<PgPool>) -> Result<HttpResponse, Error>{
    println!("{:?}", req);
    return Ok(HttpResponse::Created().json(Response{
        code: "Ok".to_string(),
        message: "User created".to_string(),
        token: Some("algo".to_string()),
    }));
}


