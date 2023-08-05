//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "assets_image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(unique)]
    pub public_id: String,
    #[sea_orm(unique)]
    pub url: String,
    pub description: Option<String>,
    pub folder: Option<String>,
    pub date_added: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::blog_post::Entity")]
    BlogPost,
    #[sea_orm(has_many = "super::common_organization::Entity")]
    CommonOrganization,
    #[sea_orm(has_many = "super::podcast_podcast::Entity")]
    PodcastPodcast,
    #[sea_orm(has_many = "super::users_profile::Entity")]
    UsersProfile,
}

impl Related<super::blog_post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BlogPost.def()
    }
}

impl Related<super::common_organization::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::CommonOrganization.def()
    }
}

impl Related<super::podcast_podcast::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PodcastPodcast.def()
    }
}

impl Related<super::users_profile::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersProfile.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}