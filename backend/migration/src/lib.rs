pub use sea_orm_migration::prelude::*;

mod column;
mod iden;
mod m20221231_055926_create_schools;
mod m20230101_065739_create_studios;
mod m20230102_073106_create_time_slots;
mod m20230102_073704_create_instructors;
mod m20230102_073713_create_courses;
mod m20230102_073716_create_course_schedules;
mod m20230104_052934_create_members;
mod m20230104_092724_create_members_courses;
mod m20230104_092735_create_lessons;
mod m20230104_092740_create_attendances;

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
            Box::new(m20230104_052934_create_members::Migration),
            Box::new(m20230104_092724_create_members_courses::Migration),
            Box::new(m20230104_092735_create_lessons::Migration),
            Box::new(m20230104_092740_create_attendances::Migration),
        ]
    }
}
