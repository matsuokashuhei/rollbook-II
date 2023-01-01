pub use sea_orm_migration::prelude::*;

mod m20221231_055926_create_schools;
mod m20230101_065739_create_studios;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221231_055926_create_schools::Migration),
            Box::new(m20230101_065739_create_studios::Migration),
        ]
    }
}
