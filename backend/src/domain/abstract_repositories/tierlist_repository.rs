use async_trait::async_trait;
use crate::domain::entities::create_tierlist_entity::CreateTierlistEntity;
use crate::domain::entities::tierlist_entity::TierlistEntity;
use crate::domain::error::ApiError;

#[async_trait]
pub trait AbstractTierlistRepository: Send + Sync {
    async fn get_tierlist_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError>;
    async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<(), ApiError>;
}