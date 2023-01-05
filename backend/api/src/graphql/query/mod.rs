pub mod instructor;
pub mod member;
pub mod school;

use async_graphql::MergedObject;
use entity::async_graphql;
use instructor::InstructorQuery;
use member::MemberQuery;
use school::SchoolQuery;

#[derive(MergedObject, Default)]
pub struct Query(InstructorQuery, MemberQuery, SchoolQuery);
