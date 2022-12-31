// use sea_orm::Statement;
use sea_orm_migration::{prelude::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = sea_query::Table::create()
            .table(Schools::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Schools::Id)
                    .integer()
                    .not_null()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Schools::Name).string().not_null())
            .col(ColumnDef::new(Schools::CreatedAt).date_time().not_null().extra("DEFAULT CURRENT_TIMESTAMP".to_owned()))
            .col(ColumnDef::new(Schools::UpdatedAt).date_time().not_null().extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned()))
            .to_owned();
        manager.create_table(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(sea_query::Table::drop().table(Schools::Table).to_owned()).await.map(|_| ())
    }
}

#[derive(Iden)]
enum Schools {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}
