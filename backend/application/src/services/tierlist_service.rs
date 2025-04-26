use derive_new::new;
use domain::entities::{CreateTierlistEntity, TierlistEntity};
use domain::error::ApiError;
use domain::repositories::AbstractTierlistRepository;
use std::sync::Arc;

#[derive(new)]
pub struct TierlistService{
    repo: Arc<dyn AbstractTierlistRepository>,
}

impl TierlistService {
    pub async fn get_all_tierlists(&self) -> Result<Vec<TierlistEntity>, ApiError> {
        self.repo.get_all_tierlists().await
    }

    pub async fn get_tierlist_by_id(&self, id: &str) -> Result<TierlistEntity, ApiError> {
        self.repo.get_tierlist_by_id(id).await
    }

    pub async fn get_tierlists_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError> {
        self.repo.get_tierlist_of_user(user_id).await
    }

    pub async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<(), ApiError> {
        self.repo.create_tierlist(tierlist).await
    }
}