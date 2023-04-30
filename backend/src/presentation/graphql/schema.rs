use crate::infrastructure::databases::mysql::create_connection;

use super::{mutations::Mutation, queries::Query};
use async_graphql::{EmptySubscription, Schema};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let connection = create_connection().await.unwrap();
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(connection)
        .finish()
}
