//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "podcast_podcast_guests")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub podcast_id: i64,
    pub guest_id: i64,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::podcast_guest::Entity",
        from = "Column::GuestId",
        to = "super::podcast_guest::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PodcastGuest,
    #[sea_orm(
        belongs_to = "super::podcast_podcast::Entity",
        from = "Column::PodcastId",
        to = "super::podcast_podcast::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    PodcastPodcast,
}

impl Related<super::podcast_guest::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PodcastGuest.def()
    }
}

impl Related<super::podcast_podcast::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PodcastPodcast.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
