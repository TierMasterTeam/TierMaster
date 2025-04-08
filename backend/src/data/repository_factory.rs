use std::sync::Arc;
use mongodb::Database;
use crate::data::repositories::tierlist_repository::TierlistRepository;
use crate::domain::abstract_repositories::tierlist_repository::AbstractTierlistRepository;

pub struct RepositoryFactory{
    tierlist_repository: Arc<dyn AbstractTierlistRepository>,
}

impl RepositoryFactory{
    pub fn init(database: &Database) -> Self {
        Self {
            tierlist_repository: Arc::new(TierlistRepository::new(database)),
        }
    }

    pub fn tierlist_repository(&self) -> Arc<dyn AbstractTierlistRepository> {
        self.tierlist_repository.clone()
    }
}