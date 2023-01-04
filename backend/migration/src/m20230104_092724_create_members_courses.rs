use crate::{
    column,
    iden::{Courses, Members, MembersCourses},
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
                    .table(MembersCourses::Table)
                    .if_not_exists()
                    .col(&mut column::define_id(MembersCourses::Id))
                    .col(
                        ColumnDef::new(MembersCourses::MemberId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(MembersCourses::CourseId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(MembersCourses::StartDate).date().not_null())
                    .col(ColumnDef::new(MembersCourses::EndDate).date().not_null())
                    .col(&mut column::define_created_at())
                    .col(&mut column::define_updated_at())
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_members_course_member")
                            .from(MembersCourses::Table, MembersCourses::MemberId)
                            .to(Members::Table, Members::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("FK_members_course_course")
                            .from(MembersCourses::Table, MembersCourses::CourseId)
                            .to(Courses::Table, Courses::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MembersCourses::Table).to_owned())
            .await
    }
}
