use std::env;
use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use application::AppState;
use crate::controllers::TierlistController;

pub struct Server;

impl Server {
    pub async fn init(state: AppState) {
        let ip = env::var("SERVER_IP").unwrap_or("127.0.0.1".to_string());
        let port = env::var("SERVER_PORT").unwrap_or("3000".to_string());

        let app = routes()
            .with_state(Arc::new(state));

        let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
            .await.unwrap();

        println!("Server successfully started on http://{ip}:{port}");
        axum::serve(listener, app).await.unwrap();
    }
}


fn routes() -> Router<Arc<AppState>> {
    let api = Router::new()
        .route("/", get(|| async {
            let version = env!("CARGO_PKG_VERSION");
            format!("Welcome to TierMaster's API\nAPI Version: {version}")
        }))
        .merge(TierlistController::get_router());

    Router::new()
        .nest("/api", api)
}