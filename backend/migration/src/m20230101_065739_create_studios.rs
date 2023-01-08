use crate::column;

use super::iden::{Schools, Studios};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Studios::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(Studios::Id))
                    .col(ColumnDef::new(Studios::SchoolId).integer().not_null())
                    .col(ColumnDef::new(Studios::Name).string().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_studio_school")
                            .from(Studios::Table, Studios::SchoolId)
                            .to(Schools::Table, Schools::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Studios::Table).to_owned())
            .await
    }
}
