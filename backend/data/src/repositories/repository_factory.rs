use std::sync::Arc;
use mongodb::Database;
use domain::repositories::{AbstractRepositoryFactory, AbstractTierlistRepository};
use crate::repositories::tierlist_repository::TierlistRepository;

pub struct RepositoryFactory{
    tierlist: Arc<TierlistRepository>
}

impl RepositoryFactory {
    pub fn init(db: &Database) -> Self {
        RepositoryFactory {
            tierlist: Arc::new(TierlistRepository::new(db)),
        }
    }
}

impl AbstractRepositoryFactory for RepositoryFactory {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository> {
        self.tierlist.clone()
    }
}