use crate::domain::entities::Entity;
use serde::de::DeserializeOwned;

pub trait Model<E: Entity> : DeserializeOwned + Unpin + Send + Sync {
    fn to_entity(self) -> E;
    fn from_entity(entity: E) -> Self;
}