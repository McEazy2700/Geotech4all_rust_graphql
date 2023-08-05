use async_graphql::*;
use sea_orm::{ActiveValue::Set, EntityTrait};
use bcrypt::{DEFAULT_COST, hash};
use entity::{users_customuser, users_profile, prelude::{UsersCustomuser, UsersProfile}};
use chrono::prelude::*;

use crate::graphql::{common::types::Success, context::AppContext};

use super::{types::{Register, RegisterInput}, authentication::generate_token};

#[derive(Default)]
pub struct UsersMutations;


#[Object]
impl UsersMutations {
    async fn register<'ctx>(&self, ctx: &Context<'ctx>, input: RegisterInput) -> Result<Success<Register>, Error> {
        let cx = ctx.data::<AppContext>()?;
        
        if input.password1 == input.password2 {
            let passwd_hash = hash(input.password1, DEFAULT_COST)?;
            
            let new_user = users_customuser::ActiveModel {
                email: Set(input.email),
                password: Set(passwd_hash),
                is_staff: Set(false),
                is_superuser: Set(false),
                is_active: Set(false),
                date_joined: Set(Utc::now().into()),
                category: Set(String::from("user")),
                ..Default::default()
            };

            let created_user = UsersCustomuser::insert(new_user)
                .exec_with_returning(&cx.db)
                .await?;
            
            let new_user_profile = users_profile::ActiveModel {
                user_id: Set(created_user.id),
                ..Default::default()
            };

            UsersProfile::insert(new_user_profile).exec(&cx.db).await?;

            let token = generate_token(&created_user)?;
            
            Ok(Success::<Register> {
                data: Some(Register {
                    token: token.token,
                    refresh_token: token.refresh_token
                }),
                success: true
            })
        } else {
            Err(Error::from("Passwords do not match"))
        }
    }
}
