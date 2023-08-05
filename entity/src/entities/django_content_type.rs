//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "django_content_type")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub app_label: String,
    pub model: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth_permission::Entity")]
    AuthPermission,
    #[sea_orm(has_many = "super::django_admin_log::Entity")]
    DjangoAdminLog,
}

impl Related<super::auth_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthPermission.def()
    }
}

impl Related<super::django_admin_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DjangoAdminLog.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
