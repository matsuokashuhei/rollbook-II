use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Instructors::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Instructors::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Instructors::Name).string().not_null())
                    .col(ColumnDef::new(Instructors::Email).string().not_null())
                    .col(ColumnDef::new(Instructors::PhoneNumber).string().not_null())
                    .col(
                        ColumnDef::new(Instructors::CreatedAt)
                            .date_time()
                            .not_null()
                            .extra("DEFAULT CURRENT_TIMESTAMP".to_owned()),
                    )
                    .col(
                        ColumnDef::new(Instructors::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra(
                                "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned(),
                            ),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Instructors::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Instructors {
    Table,
    Id,
    Name,
    Email,
    PhoneNumber,
    CreatedAt,
    UpdatedAt,
}
