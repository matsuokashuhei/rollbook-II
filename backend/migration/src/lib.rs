pub use sea_orm_migration::prelude::*;

mod m20230418_173139_create_schools;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230418_173139_create_schools::Migration)]
    }
}
