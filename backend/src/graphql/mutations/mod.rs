use self::school::SchoolMutation;
use async_graphql::MergedObject;

pub mod school;

#[derive(Default, MergedObject)]
pub struct Mutation(SchoolMutation);
