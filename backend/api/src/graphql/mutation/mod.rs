pub mod course;
pub mod instructor;
pub mod member;
pub mod members_course;
pub mod school;
pub mod studio;

use course::CourseMutation;
use entity::async_graphql;
use instructor::InstructorMutation;
use member::MemberMutation;
use members_course::MembersCourseMutation;
use school::SchoolMutation;
use studio::StudioMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    CourseMutation,
    InstructorMutation,
    MemberMutation,
    MembersCourseMutation,
    SchoolMutation,
    StudioMutation,
);
