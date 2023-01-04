use crate::column;

// use sea_orm::Statement;
use super::iden::Schools;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let stmt = sea_query::Table::create()
            .table(Schools::Table)
            .if_not_exists()
            .col(&mut column::define_id(Schools::Id))
            .col(ColumnDef::new(Schools::Name).string().not_null())
            .col(&mut column::define_created_at())
            .col(&mut column::define_updated_at())
            .to_owned();
        manager.create_table(stmt).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(Schools::Table).to_owned())
            .await
            .map(|_| ())
    }
}
