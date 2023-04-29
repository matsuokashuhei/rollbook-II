mod application;
mod domain;
// mod entity;
mod graphql;
mod infrastructure;

use anyhow::Result;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use graphql::schema::{build_schema, AppSchema};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

async fn create_app() -> Router {
    let schema = build_schema().await;
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);
    let app = Router::new()
        .route("/", get(graphiql_handler).post(graphql_handler))
        .layer(cors)
        .layer(Extension(schema));
    app
}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:4000")
            .finish(),
    )
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = create_app().await;
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    Ok(())
}
