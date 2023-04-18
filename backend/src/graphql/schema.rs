use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use super::queries::Query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish()
}
