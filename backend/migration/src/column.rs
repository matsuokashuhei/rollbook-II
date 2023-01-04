use sea_orm_migration::prelude::*;

#[derive(Iden)]
enum Column {
    Id,
    CreatedAt,
    UpdatedAt,
}

pub fn define_id<T>(name: T) -> ColumnDef
where
    T: IntoIden,
{
    let mut binding = ColumnDef::new(name);
    binding
        .integer()
        .not_null()
        .auto_increment()
        .primary_key()
        .clone()
}

pub fn define_created_at() -> ColumnDef {
    ColumnDef::new(Column::CreatedAt)
        .date_time()
        .not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP".to_owned())
        .clone()
}

pub fn define_updated_at() -> ColumnDef {
    ColumnDef::new(Column::UpdatedAt)
        .date_time()
        .not_null()
        .extra("DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP".to_owned())
        .clone()
}
