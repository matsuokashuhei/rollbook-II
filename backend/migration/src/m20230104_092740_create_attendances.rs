use crate::{
    column,
    iden::{Attendances, Lessons, Members},
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
                    .table(Attendances::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(Attendances::Id))
                    .col(ColumnDef::new(Attendances::LessonId).integer().not_null())
                    .col(ColumnDef::new(Attendances::MemberId).integer().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_attendance_lesson")
                            .from(Attendances::Table, Attendances::LessonId)
                            .to(Lessons::Table, Lessons::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_attendance_member")
                            .from(Attendances::Table, Attendances::MemberId)
                            .to(Members::Table, Members::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Attendances::Table).to_owned())
            .await
    }
}
