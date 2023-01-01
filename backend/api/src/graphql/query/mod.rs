use async_graphql::MergedObject;
use entity::async_graphql;
pub mod school;
pub use school::SchoolQuery;

#[derive(MergedObject, Default)]
pub struct Query(SchoolQuery);
