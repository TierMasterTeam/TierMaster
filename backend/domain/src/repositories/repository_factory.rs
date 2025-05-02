use crate::repositories::{AbstractAuthRepository, AbstractRedisRepository, AbstractTierlistRepository};
use std::sync::Arc;

pub trait AbstractRepositoryFactory {
    fn tierlist(&self) -> Arc<dyn AbstractTierlistRepository>;
    fn auth(&self) -> Arc<dyn AbstractAuthRepository>;
    fn redis(&self) -> Arc<dyn AbstractRedisRepository>;
}