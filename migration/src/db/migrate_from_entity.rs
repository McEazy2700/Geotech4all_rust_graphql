use sea_orm::{DbConn, EntityTrait, Schema, ConnectionTrait};

pub async fn create_table<E: EntityTrait>(db: &DbConn, entity: E) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt = builder.build(schema.create_table_from_entity(entity).if_not_exists());

    match db.execute(stmt).await {
        Ok(_) => println!("Migrated {}", entity.table_name()),
        Err(e) => println!("Migration Err: {e}")
    }
}
