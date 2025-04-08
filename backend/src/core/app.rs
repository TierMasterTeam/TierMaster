use crate::core::app_state::{AppState};
use crate::core::db::DatabaseConnection;
use crate::core::server::Server;
use dotenv::dotenv;
use mongodb::Database;

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
        let state = AppState::init(database);

        //3- Server initialisation
        Server::init(state).await;
    }
}