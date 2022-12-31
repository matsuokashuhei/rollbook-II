use crate::graphql::query::Query;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use entity::async_graphql;
use sea_orm::DatabaseConnection;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn build_schema(conn: DatabaseConnection) -> AppSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .data(conn) // placeholder for DbConn
        .finish()
}
