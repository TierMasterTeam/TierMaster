use std::env;
use mongodb::{Client};
use mongodb::options::{AuthMechanism, ClientOptions, Credential, ServerApi, ServerApiVersion};
use application::AppState;
use crate::error::DatabaseError;
use crate::repositories::RepositoryFactory;

pub struct Database;

impl Database {
    pub async fn connect() -> Result<AppState, DatabaseError> {
        let db = MongoDB::connect()
            .await?;

        let factory = RepositoryFactory::init(&db.db());

        Ok(AppState::new(Box::new(factory)))
    }
}


pub struct MongoDB(mongodb::Database);

impl MongoDB {
    pub async fn connect() -> Result<Self, DatabaseError> {
        let uri = env::var("DATABASE_URL")
            .unwrap_or("mongodb://localhost:27017".to_string());
        let db_name = env::var("MONGO_DATABASE")
            .unwrap_or("MongoDB".to_string());
        let user = env::var("MONGO_USER")
            .unwrap_or_default();
        let password = env::var("MONGO_PASSWORD")
            .unwrap_or_default();

        let mut options = ClientOptions::parse(uri).await?;

        let credentials = Credential::builder()
            .username(user)
            .password(password)
            .mechanism(AuthMechanism::ScramSha256)
            .build();

        let server_api = ServerApi::builder()
            .version(ServerApiVersion::V1)
            .build();

        options.server_api = Some(server_api);
        options.credential = Some(credentials);

        let client = Client::with_options(options)?;
        
        let database = client.database(&db_name);
        
        println!("Successfully connected to MongoDB");
        Ok(Self(database))
    }

    pub fn db(&self) -> &mongodb::Database {
        &self.0
    }
}