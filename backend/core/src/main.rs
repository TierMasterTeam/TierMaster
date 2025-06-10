use data::databases::Databases;
use dotenvy::dotenv;
use presentation::server::Server;

struct App;

impl App {
    pub async fn run() {
        //1- Fetching .env file infos
        dotenv().ok();

        println!("Starting server...");
        //2- Databases & AppState initialisation
        let app_state = Databases::connect()
            .await.unwrap();

        //3- Server initialisation
        Server::init(app_state).await
    }
}

#[tokio::main]
async fn main() {
    App::run().await
}
