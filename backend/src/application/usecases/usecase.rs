use rocket::async_trait;
use crate::domain::entities::Entity;
use crate::domain::errors::Error;

#[async_trait]
pub trait UseCase<E: Entity, T: Error>: Send + Sync {
    async fn execute(&self) -> Result<E, T>;
}