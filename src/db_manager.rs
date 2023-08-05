use entity::prelude::*;
use migration::db::migrate_from_entity::create_table;
use sea_orm::DatabaseConnection;

pub async fn perform_migrations_from_entities(db: &DatabaseConnection) {
    create_table(db, AssetsFile).await;
    create_table(db, AssetsImage).await;
    create_table(db, AssetsTag).await;
    create_table(db, UsersCustomuser).await;
    create_table(db, UsersProfile).await;
    create_table(db, UsersStaff).await;
}
