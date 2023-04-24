use super::{mutations::Mutation, queries::Query};
use async_graphql::{EmptySubscription, Schema};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription).finish()
}
