use entity::async_graphql;
pub mod school;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(school::SchoolMutation);
