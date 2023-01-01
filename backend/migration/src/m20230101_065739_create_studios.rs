use crate::m20221231_055926_create_schools::Schools;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Studios::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Studios::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Studios::SchoolId).integer().not_null())
                    .col(ColumnDef::new(Studios::Name).string().not_null())
                    .col(
                        ColumnDef::new(Studios::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Studios::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra(
                                "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned(),
                            ),
                    )
                    .foreign_key(
                        ForeignKeyCreateStatement::new()
                            .name("FK_studio_school")
                            .from_tbl(Studios::Table)
                            .from_col(Studios::SchoolId)
                            .to_tbl(Schools::Table)
                            .to_col(Schools::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Studios::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Studios {
    Table,
    Id,
    SchoolId,
    Name,
    CreatedAt,
    UpdatedAt,
}
