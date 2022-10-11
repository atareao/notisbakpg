use actix_web_httpauth::extractors::bearer::BearerAuth;
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use crate::routes::users::Claims;



pub fn test(credentials: BearerAuth){
    eprintln!("{}", credentials.token());
    let decoded = decode::<Claims>(credentials.token(), &DecodingKey::from_secret("SECRETO".as_ref()), &Validation::default());
    let header = decode_header(credentials.token()).unwrap();
    let jwt = header.jwk;
    eprintln!("BearerAuth {:?}", credentials);
    eprintln!("BearerAuth {:?}", decoded);
    eprintln!("Sub: {}", decoded.unwrap().claims.sub);
    eprintln!("decoded {:?}", jwt);
}
