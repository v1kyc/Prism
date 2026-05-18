use crate::rxtx::database::Database;
use crate::router::router;
mod router;
mod rxtx;
mod tools;
pub mod auth;
pub mod jobs;

#[tokio::main]
async fn main() {
    let db = Database::new()
        .await
        .expect("Failed to connect to database");

    let tcp_listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    axum::serve(tcp_listener, router(db)).await.unwrap();
}