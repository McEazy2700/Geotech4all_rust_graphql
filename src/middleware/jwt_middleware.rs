use actix_web::{
    dev::Payload, error::ErrorUnauthorized, http, web, Error, FromRequest, HttpRequest,
};
use std::future::{ready, Ready};

use crate::graphql::{
    context::AppContext,
    users::{authentication::verify_token, types::User},
};
use futures::executor::block_on;


#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user: Option<User>,
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req.headers().get(http::header::AUTHORIZATION);
        let auth_token = match auth_header {
            Some(header) => String::from(header.to_str().unwrap_or("")),
            None => String::from(""),
        };
        if auth_token.is_empty() {
            return ready(Ok(AuthUser { user: None }));
        } else {
            let claims_result = verify_token(auth_token);
            match claims_result {
                Ok(claims) => {
                    let ctx = req.app_data::<web::Data<AppContext>>().unwrap();
                    let user = block_on(User::get(&ctx.db, claims.custom.id));
                    match user {
                        Ok(usr) => ready(Ok(AuthUser { user: Some(usr) })),
                        Err(_) => ready(Err(ErrorUnauthorized("Invalid auth token"))),
                    }
                }
                Err(_) => ready(Err(ErrorUnauthorized("Invalid auth token"))),
            }
        }
    }
}
