use async_trait::async_trait;
use crate::entities::CreateTierlistEntity;
use crate::entities::TierlistEntity;
use crate::error::ApiError;

#[async_trait]
pub trait AbstractTierlistRepository: Send + Sync {
    async fn get_tierlist_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError>;
    async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<(), ApiError>;
    async fn get_tierlist_by_id(&self, id: &str) -> Result<TierlistEntity, ApiError>;
}