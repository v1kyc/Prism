// Imports
use crate::router::router;
mod router;
mod rxtx;
mod tools;
pub mod jobs;

#[tokio::main]
async fn main() {
    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(tcp_listener, router()).await.unwrap();
}
