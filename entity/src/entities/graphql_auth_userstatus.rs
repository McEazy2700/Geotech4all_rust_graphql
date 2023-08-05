//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "graphql_auth_userstatus")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub verified: bool,
    pub archived: bool,
    pub secondary_email: Option<String>,
    #[sea_orm(unique)]
    pub user_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users_customuser::Entity",
        from = "Column::UserId",
        to = "super::users_customuser::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UsersCustomuser,
}

impl Related<super::users_customuser::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersCustomuser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
