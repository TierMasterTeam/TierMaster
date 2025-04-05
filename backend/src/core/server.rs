use crate::presentation::controllers::controller::{Controller, Route};
use crate::presentation::controllers::tierlist_controller::TierListController;
use axum::{
    routing::get,
    Router,
};
use std::env;

pub struct Server;

impl Server {
    pub async fn init() {
        let ip = env::var("SERVER_IP").unwrap_or("127.0.0.1".to_string());
        let port = env::var("SERVER_PORT").unwrap_or("3000".to_string());

        let mut app = Router::new()
            .route("/", get(|| async {
                let version = env!("CARGO_PKG_VERSION");
                format!("Welcome to TierMaster's API\nAPI Version: {version}")
            }));

        let tierlist_controller = TierListController;
        app = add_routes(app, &tierlist_controller);

        let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
            .await.unwrap();

        println!("Server successfully started on http://{ip}:{port}");
        axum::serve(listener, app).await.unwrap();
    }
}

fn add_routes(mut app: Router, controller: &dyn Controller<()>) -> Router {
    for route in controller.get_routes() {
        app = app.route(route.path(), route.method());
    }
    app
}