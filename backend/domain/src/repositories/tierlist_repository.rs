use crate::entities::CreateTierlistEntity;
use crate::entities::TierlistEntity;
use crate::entities::UpdateTierlistEntity;
use crate::error::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait AbstractTierlistRepository: Send + Sync {
    async fn get_all_tierlists(&self) -> Result<Vec<TierlistEntity>, ApiError>;
    async fn get_tierlist_by_id(&self, id: &str, user_id: Option<String>) -> Result<TierlistEntity, ApiError>;
    async fn get_tierlist_of_user(&self, user_id: &str, can_see_private_tierlists: bool) -> Result<Vec<TierlistEntity>, ApiError>;

    async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<String, ApiError>;

    async fn update_tierlist_by_id(&self, id: &str, tierlist: UpdateTierlistEntity) -> Result<(), ApiError>;

    async fn delete_by_id(&self, id: &str) -> Result<(), ApiError>;
}