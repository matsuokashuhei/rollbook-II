use crate::{
    m20230102_073106_create_time_slots::TimeSlots, m20230102_073713_create_courses::Courses,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(CourseSchedules::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(CourseSchedules::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(CourseSchedules::CourseId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(CourseSchedules::TimeSlotId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(CourseSchedules::StartDate).date().not_null())
                    .col(ColumnDef::new(CourseSchedules::EndDate).date().not_null())
                    .col(
                        ColumnDef::new(CourseSchedules::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(CourseSchedules::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra(
                                "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned(),
                            ),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_course_schedule_course")
                            .from_tbl(CourseSchedules::Table)
                            .from_col(CourseSchedules::CourseId)
                            .to_tbl(Courses::Table)
                            .to_col(Courses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_course_schedule_time_slot")
                            .from_tbl(CourseSchedules::Table)
                            .from_col(CourseSchedules::TimeSlotId)
                            .to_tbl(TimeSlots::Table)
                            .to_col(TimeSlots::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(CourseSchedules::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum CourseSchedules {
    Table,
    Id,
    CourseId,
    TimeSlotId,
    StartDate,
    EndDate,
    CreatedAt,
    UpdatedAt,
}
