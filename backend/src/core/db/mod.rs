pub mod mongo_connection;


use std::error::Error;
use async_trait::async_trait;

pub enum DatabaseType{
    MONGODB,
    REDIS,
}

#[async_trait]
pub trait DatabaseConnection: Sized {
    async fn connect() -> Result<Self, Box<dyn Error + Send + Sync>>;
}