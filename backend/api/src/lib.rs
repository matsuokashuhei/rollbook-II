mod graphql;
use async_graphql_axum::GraphQLRequest;
use async_graphql_axum::GraphQLResponse;
use axum::response::Html;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use entity::async_graphql::http::GraphiQLSource;
use graphql::schema::AppSchema;
use graphql::schema::build_schema;
use sea_orm::Database;
use std::net::SocketAddr;
use tower::ServiceBuilder;

async fn graphql_handler(
    schema: Extension<AppSchema>,
    req: GraphQLRequest
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:3000")
            .finish(),
    )
}

#[tokio::main]
async fn start() {
    let conn = Database::connect(std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let schema = build_schema(conn).await;
    let app = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .layer(ServiceBuilder::new().layer(Extension(schema)));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

pub fn main() {
    start()
}
