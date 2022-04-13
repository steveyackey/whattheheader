use axum::{body::Body, http::Request, routing::get, Router};
use std::net::SocketAddr;

async fn yo(request: Request<Body>) -> String {
    tracing::info!("req: {:#?}", request);
    format!("Headers: {:#?}", request)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(yo));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8082));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

