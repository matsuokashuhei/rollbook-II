mod graphql;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::Method,
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use entity::async_graphql::http::GraphiQLSource;
use graphql::schema::{build_schema, AppSchema};
use sea_orm::Database;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:4000")
            .finish(),
    )
}

#[tokio::main]
async fn start() {
    let conn = Database::connect(std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let schema = build_schema(conn).await;
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers(Any)
        .allow_methods(Any);
    // .allow_methods([Method::GET, Method::POST]);
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(ServiceBuilder::new().layer(Extension(schema)).layer(cors));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn main() {
    start()
}
