use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn start() {
    let app = Router::new().route("/", get(root));
    let addr = SocketAddr::from(([0,0,0,0], 3000));
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn root() -> &'static str {
    "hello world"
}

pub fn main() {
    start()
}
