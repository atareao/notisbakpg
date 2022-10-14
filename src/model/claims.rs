use serde::{Serialize, Deserialize};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{encode, decode, decode_header, EncodingKey, DecodingKey,
                   Header, Validation};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

impl Claims{
    pub fn new(index: i32)-> Self{
        let expiration: usize = env::var("EXPIRATION")
            .expect("EXPIRATION not set")
            .parse()
            .unwrap();
        let exp = usize::try_from(
            jsonwebtoken::get_current_timestamp()).unwrap() + expiration;
        Self{
            sub: index.to_string(),
            exp,
        }
    }
    pub fn get_token(&self) -> Result<String, std::io::Error>{
        let secret = env::var("SECRET").unwrap();
        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(secret.as_ref())).unwrap()
        )
    }
    pub fn get_index(credentials: BearerAuth) -> Result<i32, std::io::Error>{
        let secret = env::var("SECRET").unwrap();
        let decoded = decode::<Claims>(
            credentials.token(),
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default()
        );
        Ok(decoded.unwrap().claims.sub.parse().unwrap())
    }
}

#[derive(Serialize, Deserialize)]
struct Response{
    code: String,
    message: String,
    token: Option<String>,
}

pub fn get_user_id_from_token(credentials: BearerAuth) -> Result<i32, std::io::Error>{
    let secret = env::var("SECRET").unwrap();
    let decoded = decode::<Claims>(credentials.token(), &DecodingKey::from_secret(secret.as_bytes()), &Validation::default());
    Ok(decoded.unwrap().claims.sub.parse().unwrap())
}
