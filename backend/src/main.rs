use axum::routing::get;
use axum::Router;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let ip = env::var("SERVER_IP").unwrap_or("127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or("8080".to_string());

    let app = Router::new()
        .route("/", get(|| async {
            let version = env!("CARGO_PKG_VERSION");
            format!("Welcome to TierMaster's API\nAPI Version: {version}")
        }));

    let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
        .await.unwrap();

    println!("Server listening at http://{ip}:{port}");
    axum::serve(listener, app).await.unwrap();
}