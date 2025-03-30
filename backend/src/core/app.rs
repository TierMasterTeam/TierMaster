use dotenv::dotenv;
use mongodb::Database;
use crate::core::app_state::{AppState, APP_STATE};
use crate::core::db::DatabaseConnection;
use crate::core::server::Server;

pub struct App;

impl App {
    /// Setups and starts the application
    pub async fn run() {
        //1- Fetching .env file infos
        dotenv().ok();

        //2- Databases initialisation
        let database = Database::connect()
            .await
            .unwrap();

        //4- AppState initialisation
        let state = AppState::new(database);
        APP_STATE.set(state).unwrap();

        //3- Server initialisation
        Server::init().await;
    }
}