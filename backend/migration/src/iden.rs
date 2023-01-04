use sea_orm_migration::prelude::*;

// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
pub enum Schools {
    Table,
    Id,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Studios {
    Table,
    Id,
    SchoolId,
    Name,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum TimeSlots {
    Table,
    Id,
    StudioId,
    DayOfWeek,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}

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

#[derive(Iden)]
pub enum CourseSchedules {
    Table,
    Id,
    CourseId,
    TimeSlotId,
    StartDate,
    EndDate,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum MembersCourses {
    Table,
    Id,
    MemberId,
    CourseId,
    StartDate,
    EndDate,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Members {
    Table,
    Id,
    Number,
    Name,
    Kana,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Lessons {
    Table,
    Id,
    CourseScheduleId,
    Date,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Attendances {
    Table,
    Id,
    LessonId,
    MemberId,
    CreatedAt,
    UpdatedAt,
}
