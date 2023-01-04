use super::{
    column,
    iden::{CourseSchedules, Lessons},
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
                    .table(Lessons::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(Lessons::Id))
                    .col(
                        ColumnDef::new(Lessons::CourseScheduleId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Lessons::Date).date().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_lesson_course_schedule")
                            .from(Lessons::Table, Lessons::CourseScheduleId)
                            .to(CourseSchedules::Table, CourseSchedules::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Lessons::Table).to_owned())
            .await
    }
}
