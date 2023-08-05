pub use sea_orm_migration::prelude::*;

pub mod db;

mod m20230805_072605_create_version_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230805_072605_create_version_table::Migration),
        ]
    }
}
