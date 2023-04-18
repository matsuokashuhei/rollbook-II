use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::create()
            .table(School::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(School::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(School::Name).string().not_null())
            .col(
                ColumnDef::new(School::CreatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
            )
            .col(
                ColumnDef::new(School::UpdatedAt)
                    .date_time()
                    .not_null()
                    .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned()),
            )
            .to_owned();
        manager.create_table(stmt).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = Table::drop().table(School::Table).to_owned();
        manager.drop_table(stmt).await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
#[iden = "schools"]
enum School {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
