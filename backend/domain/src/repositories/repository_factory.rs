use std::sync::Arc;
use crate::repositories::AbstractTierlistRepository;

pub trait AbstractRepositoryFactory {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository>;
}