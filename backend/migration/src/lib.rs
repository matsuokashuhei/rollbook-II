pub use sea_orm_migration::prelude::*;

mod m20221231_055926_create_schools;
mod m20230101_065739_create_studios;
mod m20230102_073106_create_time_slots;
mod m20230102_073704_create_instructors;
mod m20230102_073713_create_courses;
mod m20230102_073716_create_course_schedules;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20221231_055926_create_schools::Migration),
            Box::new(m20230101_065739_create_studios::Migration),
            Box::new(m20230102_073106_create_time_slots::Migration),
            Box::new(m20230102_073704_create_instructors::Migration),
            Box::new(m20230102_073713_create_courses::Migration),
            Box::new(m20230102_073716_create_course_schedules::Migration),
        ]
    }
}
