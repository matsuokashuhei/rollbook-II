use crate::column;

use super::iden::Members;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Members::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(Members::Id))
                    .col(ColumnDef::new(Members::Number).integer().not_null())
                    .col(ColumnDef::new(Members::Name).string().not_null())
                    .col(ColumnDef::new(Members::Kana).string().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .index(
                        Index::create()
                            .unique()
                            .name("IDX_number_in_members")
                            .col(Members::Number),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Members::Table).to_owned())
            .await
    }
}
