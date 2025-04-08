use std::sync::Arc;
use derive_new::new;
use crate::domain::abstract_repositories::tierlist_repository::AbstractTierlistRepository;
use crate::domain::entities::create_tierlist_entity::CreateTierlistEntity;
use crate::domain::entities::tierlist_entity::TierlistEntity;
use crate::domain::error::ApiError;

#[derive(new)]
pub struct TierlistService{
    repo: Arc<dyn AbstractTierlistRepository>,
}

impl TierlistService {
    pub async fn get_tierlists_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError> {
        self.repo.get_tierlist_of_user(user_id).await
    }

    pub async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<(), ApiError> {
        self.repo.create_tierlist(tierlist).await
    }
}