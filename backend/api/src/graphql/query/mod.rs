use async_graphql::MergedObject;
use entity::async_graphql;
use school::SchoolQuery;
pub mod school;

#[derive(MergedObject, Default)]
pub struct Query(SchoolQuery);
