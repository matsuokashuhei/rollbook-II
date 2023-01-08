use crate::{column, iden::Instructors};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Instructors::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(Instructors::Id))
                    .col(ColumnDef::new(Instructors::Name).string().not_null())
                    .col(ColumnDef::new(Instructors::Email).string().not_null())
                    .col(ColumnDef::new(Instructors::PhoneNumber).string().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .index(
                        Index::create()
                            .unique()
                            .name("IDX_name_in_instructors")
                            .col(Instructors::Name),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Instructors::Table).to_owned())
            .await
    }
}
