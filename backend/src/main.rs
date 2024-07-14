mod controller;
mod globals;
mod middleware;
mod routes;

use dotenv::dotenv;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(routes::get_router()).await;
}
