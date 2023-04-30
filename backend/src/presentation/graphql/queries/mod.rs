pub mod school;

use async_graphql::MergedObject;
use school::SchoolQuery;

#[derive(MergedObject, Default)]
pub struct Query(SchoolQuery);
