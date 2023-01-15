use std::{net::SocketAddr, env};
use axum::{response::Html, routing::get, Router};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new().route("/", get(root));
    let addr = env::var("SERVER_ADDRESS")
        .unwrap_or_else(|e| panic!("failed to get env with name socked 'SERVER_ADDRESS': {:?}", e));

    let parsed_address: SocketAddr = addr
        .parse()
        .unwrap_or_else(|e| panic!("failed to parse socket address {}: {}", addr, e));

    println!("listening on {}", parsed_address);
    println!("{}", addr.to_string());

    axum::Server::bind(&parsed_address)
        .serve(app.into_make_service())
        .await
        .unwrap();

}

async fn root() -> Html<&'static str> {
    Html("Hello world! and hello world")
}