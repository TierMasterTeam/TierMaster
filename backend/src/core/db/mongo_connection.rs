use crate::core::db::DatabaseConnection;
use async_trait::async_trait;
use mongodb::options::{ClientOptions, Credential, ServerApi, ServerApiVersion};
use mongodb::{Client, Database};
use std::env;
use std::error::Error;

#[async_trait]
impl DatabaseConnection for Database {
    async fn connect() -> Result<Self, Box<dyn Error + Send + Sync>> {
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
            .build();

        let server_api = ServerApi::builder()
            .version(ServerApiVersion::V1)
            .build();

        options.server_api = Some(server_api);
        options.credential = Some(credentials);

        let client = Client::with_options(options)?;

        let database = client.database(&db_name);

        println!("Successfully connected to MongoDB");
        Ok(database)
    }
}