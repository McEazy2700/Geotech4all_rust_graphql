//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "blog_post")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub title: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub r#abstract: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    pub likes: i32,
    pub dislikes: i32,
    #[sea_orm(column_type = "Double", nullable)]
    pub read_length: Option<f64>,
    pub date_added: DateTimeWithTimeZone,
    pub last_updated: DateTimeWithTimeZone,
    pub author_id: i64,
    pub cover_photo_id: Option<i64>,
    pub views: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::assets_image::Entity",
        from = "Column::CoverPhotoId",
        to = "super::assets_image::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    AssetsImage,
    #[sea_orm(has_many = "super::blog_comment::Entity")]
    BlogComment,
    #[sea_orm(
        belongs_to = "super::users_customuser::Entity",
        from = "Column::AuthorId",
        to = "super::users_customuser::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UsersCustomuser,
}

impl Related<super::assets_image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AssetsImage.def()
    }
}

impl Related<super::blog_comment::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlogComment.def()
    }
}

impl Related<super::users_customuser::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersCustomuser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
