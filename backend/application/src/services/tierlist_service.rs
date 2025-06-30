use derive_new::new;
use domain::entities::{CreateTierlistEntity, TierlistEntity, UpdateTierlistEntity};
use domain::error::ApiError;
use domain::repositories::AbstractTierlistRepository;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct TierlistService{
    repo: Arc<dyn AbstractTierlistRepository>,
}

impl TierlistService {
    pub async fn get_all_tierlists(&self) -> Result<Vec<TierlistEntity>, ApiError> {
        self.repo.get_all_tierlists().await
    }

    pub async fn get_tierlist_by_id(&self, id: &str, user_id: Option<String>) -> Result<TierlistEntity, ApiError> {
        self.repo.get_tierlist_by_id(id, user_id).await
    }

    pub async fn get_tierlists_of_user(&self, user_id: &str, can_see_private_tierlists: bool) -> Result<Vec<TierlistEntity>, ApiError> {
        self.repo.get_tierlist_of_user(user_id, can_see_private_tierlists).await
    }

    pub async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<String, ApiError> {
        self.repo.create_tierlist(tierlist).await
    }

    pub async fn update_tierlist_by_id(&self, id: &str, tierlist: UpdateTierlistEntity) -> Result<(), ApiError>  {
        self.repo.update_tierlist_by_id(id, tierlist).await
    }
    
    pub async fn delete_by_id(&self, id: &str) -> Result<(), ApiError> {
        self.repo.delete_by_id(id).await
    }
}
