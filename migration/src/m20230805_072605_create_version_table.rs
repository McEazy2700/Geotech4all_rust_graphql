use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Version::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Version::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment()
                    ).col(ColumnDef::new(Version::Number).float().not_null())
                    .to_owned()
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager.drop_table(
            Table::drop()
                .table(Version::Table)
                .to_owned()
            ).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Version {
    Table,
    Id,
    Number
}