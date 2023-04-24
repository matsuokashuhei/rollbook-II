use anyhow::Result;
use async_graphql::{Context, Object};

use crate::graphql::objects::school::School;

#[derive(Default)]
pub struct SchoolQuery;

#[Object]
impl SchoolQuery {
    async fn schools(&self, _ctx: &Context<'_>) -> Result<Vec<School>> {
        Ok(vec![School {
            id: 1,
            name: "Tachikawa".to_owned(),
        }])
    }
}
