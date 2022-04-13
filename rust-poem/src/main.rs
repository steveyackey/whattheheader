use poem::{get, handler, listener::TcpListener, Request, Route, Server};

#[handler]
fn yo(body: &Request) -> String {
    tracing::info!("req: {:#?}", body);
    format!("{:#?}", body)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let app = Route::new().at("/", get(yo));
    Server::new(TcpListener::bind("127.0.0.1:8081"))
        .run(app)
        .await
}
