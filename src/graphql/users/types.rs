use async_graphql::*;
use sea_orm::{DatabaseConnection, EntityTrait};
use entity::users_customuser;

#[derive(SimpleObject, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub is_staff: bool,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    pub full_name: Option<String>,
}

impl User {
    pub async fn get(db: &DatabaseConnection, id: i64) -> Result<Self, Error> {
        let user = users_customuser::Entity::find_by_id(id).one(db).await?;
        match user {
            Some(user) => Ok(user.into()),
            None => Err(Error::from("user with specified id was not found")),
        }
    }
}

impl From<users_customuser::Model> for User {
    fn from(value: users_customuser::Model) -> Self {
        Self {
            id: value.id,
            email: value.email,
            is_staff: value.is_staff,
            first_name: value.first_name,
            last_name: value.last_name,
            username: value.username,
            full_name: value.full_name
        }
    }
}

#[derive(SimpleObject)]
pub struct Register {
    pub token: String,
    pub refresh_token: String,
}

#[derive(InputObject)]
pub struct RegisterInput {
    pub email: String,
    pub password1: String,
    pub password2: String
}
