use crate::controllers::{AuthController, ImageController, TemplateController, TierlistController, WebsocketController};
use application::AppState;
use axum::http::{header, Method};
use axum::routing::get;
use axum::Router;
use socketioxide::layer::SocketIoLayer;
use socketioxide::SocketIo;
use std::env;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};

pub struct Server;

impl Server {
    pub async fn init(state: AppState) {
        let ip = env::var("SERVER_IP").unwrap_or("127.0.0.1".to_string());
        let port = env::var("SERVER_PORT").unwrap_or("3000".to_string());

        // TODO set log level based on environment (debug or prod)
        tracing_subscriber::registry()
            .with(EnvFilter::new("debug"))
            .with(fmt::layer())
            .init();

        let app = routes()
            .with_state(Arc::new(state.clone()))
            .layer(
                ServiceBuilder::new()
                .layer(create_cors())
                .layer(websockets(state.clone()))
            ).layer(TraceLayer::new_for_http());

        let listener = tokio::net::TcpListener::bind(format!("{ip}:{port}"))
            .await.unwrap();

        println!("Server successfully started on http://{ip}:{port}");
        axum::serve(listener, app).await.unwrap();
    }
}

fn websockets(app_state: AppState) -> SocketIoLayer {
    let (websocket_layer, io) = SocketIo::builder()
        .with_state(app_state)
        .build_layer();

    io.ns("/api/ws", WebsocketController::on_connect);
    
    websocket_layer
}


fn routes() -> Router<Arc<AppState>> {
    let api = Router::new()
        .route("/", get(|| async {
            let version = env!("CARGO_PKG_VERSION");
            format!("Welcome to TierMaster's API\nAPI Version: {version}")
        }))
        .merge(AuthController::get_router())
        .merge(TierlistController::get_router())
        .merge(TemplateController::get_router())
        .merge(ImageController::get_router());

    Router::new()
        .nest("/api", api)
        .layer(create_cors())
}

fn create_cors() -> CorsLayer {
    let origins = [
        "http://localhost:5173".parse().unwrap(),
        "https://tiermaster.app".parse().unwrap(),
        "https://www.tiermaster.app".parse().unwrap(),
    ];

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::AUTHORIZATION,header::CONTENT_TYPE, header::ACCEPT,header::ORIGIN,])
        .allow_credentials(true)
}