mod handlers;

use axum::{routing::get, Router};
use dotenv::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Define routes
    let app = Router::new()
        .route("/", get(root))
        .route("/eth/balance/:addr", get(handlers::get_eth_balance))
        .route("/eth/tokens/:addr", get(handlers::get_token_balances));

    // Start the server using TcpListener (Axum 0.7+ idiomatic)
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Axum + Ethers API is alive!"
}
