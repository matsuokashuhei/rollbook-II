use entity::async_graphql;
use school::SchoolMutation;
use studio::StudioMutation;
pub mod school;
pub mod studio;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(SchoolMutation, StudioMutation);
