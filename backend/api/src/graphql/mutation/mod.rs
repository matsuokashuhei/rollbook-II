pub mod attendance;
pub mod course;
pub mod instructor;
pub mod lesson;
pub mod member;
pub mod members_course;
pub mod school;
pub mod studio;

use attendance::AttendanceMutation;
use course::CourseMutation;
use entity::async_graphql;
use instructor::InstructorMutation;
use lesson::LessonMutation;
use member::MemberMutation;
use members_course::MembersCourseMutation;
use school::SchoolMutation;
use studio::StudioMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    AttendanceMutation,
    CourseMutation,
    InstructorMutation,
    LessonMutation,
    MemberMutation,
    MembersCourseMutation,
    SchoolMutation,
    StudioMutation,
);
