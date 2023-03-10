use crate::column;

use super::iden::{Studios, TimeSlots};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TimeSlots::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(TimeSlots::Id))
                    .col(ColumnDef::new(TimeSlots::StudioId).integer().not_null())
                    .col(ColumnDef::new(TimeSlots::DayOfWeek).integer().not_null())
                    .col(ColumnDef::new(TimeSlots::StartTime).string().not_null())
                    .col(ColumnDef::new(TimeSlots::EndTime).string().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_time_slot_studio")
                            .from(TimeSlots::Table, TimeSlots::StudioId)
                            .to(Studios::Table, Studios::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TimeSlots::Table).to_owned())
            .await
    }
}
