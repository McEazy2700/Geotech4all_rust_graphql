//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "podcast_host")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub podcast_id: i64,
    pub user_id: i64,
    pub date_added: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::podcast_podcast::Entity",
        from = "Column::PodcastId",
        to = "super::podcast_podcast::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PodcastPodcast,
    #[sea_orm(
        belongs_to = "super::users_customuser::Entity",
        from = "Column::UserId",
        to = "super::users_customuser::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    UsersCustomuser,
}

impl Related<super::podcast_podcast::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PodcastPodcast.def()
    }
}

impl Related<super::users_customuser::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UsersCustomuser.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
