use crate::{
    m20230101_065739_create_studios::Studios, m20230102_073704_create_instructors::Instructors,
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
                    .col(
                        ColumnDef::new(Courses::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Courses::StudioId).integer().not_null())
                    .col(ColumnDef::new(Courses::Name).string().not_null())
                    .col(ColumnDef::new(Courses::Price).integer().not_null())
                    .col(ColumnDef::new(Courses::StartDate).date().not_null())
                    .col(ColumnDef::new(Courses::EndDate).date().not_null())
                    .col(ColumnDef::new(Courses::InstructorId).integer().not_null())
                    .col(
                        ColumnDef::new(Courses::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Courses::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra(
                                "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned(),
                            ),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_course_studio")
                            .from_tbl(Courses::Table)
                            .from_col(Courses::StudioId)
                            .to_tbl(Studios::Table)
                            .to_col(Studios::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_course_instructor")
                            .from_tbl(Courses::Table)
                            .from_col(Courses::InstructorId)
                            .to_tbl(Instructors::Table)
                            .to_col(Instructors::Id)
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

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Courses {
    Table,
    Id,
    StudioId,
    Name,
    InstructorId,
    Price,
    StartDate,
    EndDate,
    CreatedAt,
    UpdatedAt,
}
