use crate::m20230101_065739_create_studios::Studios;
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
                    .col(
                        ColumnDef::new(TimeSlots::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TimeSlots::StudioId).integer().not_null())
                    .col(ColumnDef::new(TimeSlots::DayOfWeek).integer().not_null())
                    .col(ColumnDef::new(TimeSlots::StartTime).string().not_null())
                    .col(ColumnDef::new(TimeSlots::EndTime).string().not_null())
                    .col(
                        ColumnDef::new(TimeSlots::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(TimeSlots::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra(
                                "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned(),
                            ),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_time_slot_studio")
                            .from_tbl(TimeSlots::Table)
                            .from_col(TimeSlots::StudioId)
                            .to_tbl(Studios::Table)
                            .to_col(Studios::Id)
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

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum TimeSlots {
    Table,
    Id,
    StudioId,
    DayOfWeek,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}
