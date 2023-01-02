pub mod instructor;
pub mod school;

use async_graphql::MergedObject;
use entity::async_graphql;
use instructor::InstructorQuery;
use school::SchoolQuery;

#[derive(MergedObject, Default)]
pub struct Query(SchoolQuery, InstructorQuery);
