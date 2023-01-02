pub mod instructor;
pub mod school;
pub mod studio;

use entity::async_graphql;
use instructor::InstructorMutation;
use school::SchoolMutation;
use studio::StudioMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(SchoolMutation, StudioMutation, InstructorMutation);
