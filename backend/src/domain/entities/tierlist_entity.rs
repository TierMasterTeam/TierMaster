use crate::domain::entities::Entity;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TierlistEntity {
    pub id: String,
    pub name: String,
    pub author: String,
}

impl Entity for TierlistEntity {}