use tokio::{self, net::TcpListener};
use tracing_subscriber::EnvFilter;

use crate::router::get_router;

pub mod champions;
mod controllers;
mod router;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let router = get_router().await;
    let listener = TcpListener::bind("0.0.0.0:3111").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
