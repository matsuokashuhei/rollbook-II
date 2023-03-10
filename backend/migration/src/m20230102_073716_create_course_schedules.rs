use super::{
    column,
    iden::{CourseSchedules, Courses, TimeSlots},
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
                    .col(&mut column::define_id(CourseSchedules::Id))
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
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_course_schedule_course")
                            .from(CourseSchedules::Table, CourseSchedules::CourseId)
                            .to(Courses::Table, Courses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_course_schedule_time_slot")
                            .from(CourseSchedules::Table, CourseSchedules::TimeSlotId)
                            .to(TimeSlots::Table, TimeSlots::Id)
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
