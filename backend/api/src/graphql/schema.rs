use crate::graphql::mutation::Mutation;
use crate::graphql::query::Query;
use async_graphql::Schema;
use entity::async_graphql::{self, EmptySubscription};
use sea_orm::DatabaseConnection;
// use tower::util::error::optional::None;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema(conn: DatabaseConnection) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(conn) // placeholder for DbConn
        .finish()
}
