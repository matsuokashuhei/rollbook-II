use crate::{
    column,
    iden::{Courses, Instructors, Studios},
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
                    .table(Courses::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(Courses::Id))
                    .col(ColumnDef::new(Courses::StudioId).integer().not_null())
                    .col(ColumnDef::new(Courses::Name).string().not_null())
                    .col(ColumnDef::new(Courses::Price).integer().not_null())
                    .col(ColumnDef::new(Courses::StartDate).date().not_null())
                    .col(ColumnDef::new(Courses::EndDate).date().not_null())
                    .col(ColumnDef::new(Courses::InstructorId).integer().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_course_studio")
                            .from(Courses::Table, Courses::StudioId)
                            .to(Studios::Table, Studios::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_course_instructor")
                            .from(Courses::Table, Courses::InstructorId)
                            .to(Instructors::Table, Instructors::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Courses::Table).to_owned())
            .await
    }
}
