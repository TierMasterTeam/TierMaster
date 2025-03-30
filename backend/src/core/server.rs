use std::env;
use axum::Router;
use axum::routing::get;
use crate::presentation::controllers::AxumController;
use crate::presentation::controllers::tl_controller::TlController;

pub struct Server;

impl Server {
    pub async fn init(){
        let ip = env::var("SERVER_IP").unwrap_or("127.0.0.1".to_string());
        let port = env::var("SERVER_PORT").unwrap_or("3000".to_string());

        let app = Self::route();

        let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
            .await.unwrap();

        println!("Server successfully started on http://{ip}:{port}");
        axum::serve(listener, app).await.unwrap();
    }


    fn route() -> Router {
        Router::new()
            .route("/", get(|| async {
                let version = env!("CARGO_PKG_VERSION");
                format!("Welcome to TierMaster's API\nAPI Version: {version}")
            }))
            .nest("/tierlist", TlController::get_router())
    }
}