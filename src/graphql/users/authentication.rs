use entity::users_customuser::Model as User;
use jwt_simple::{prelude::*, Error};
use serde::{Deserialize, Serialize};
use std::env::var;


pub struct Token {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthJWTClaims {
    pub id: i64,
    pub is_staff: Option<bool>,
}

pub fn generate_token(user: &User) -> Result<Token, Error> {
    let jwt_claims = AuthJWTClaims {
        id: user.id,
        is_staff: Some(user.is_staff),
    };
    let secret_key = var("SECRET_KEY").expect("SECRET_KEY env should be set");
    let key = HS256Key::from_bytes(secret_key.as_bytes());
    let claims = Claims::with_custom_claims(jwt_claims, Duration::from_hours(2));

    let refresh_jwt_claims = AuthJWTClaims {
        id: user.id,
        is_staff: None,
    };
    
    let refresh_claims = Claims::with_custom_claims(refresh_jwt_claims, Duration::from_days(1));
    let token = key.authenticate(claims)?;
    let refresh_token = key.authenticate(refresh_claims)?;
    Ok(Token {
        token,
        refresh_token,
    })
}

pub fn verify_token(token: String) -> Result<JWTClaims<AuthJWTClaims>, Error> {
    let secret_key = var("SECRET_KEY").expect("SECRET_KEY env should be set");
    let key = HS256Key::from_bytes(secret_key.as_bytes());
    key.verify_token::<AuthJWTClaims>(&token, None)
}
