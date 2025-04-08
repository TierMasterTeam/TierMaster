use crate::data::models::model::Model;
use crate::domain::entities::tierlist_entity::TierlistEntity;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TierlistModel {
    pub _id: ObjectId,
    pub name: String,
    pub author: ObjectId,
}

impl Model<TierlistEntity> for TierlistModel {
    fn to_entity(self) -> TierlistEntity {
        TierlistEntity {
            id: self._id.to_string(),
            name: self.name,
            author: self.author.to_string(),
        }
    }

    fn from_entity(entity: TierlistEntity) -> Self {
        TierlistModel {
            _id: ObjectId::parse_str(entity.id).unwrap(),
            name: entity.name,
            author: ObjectId::parse_str(entity.author).unwrap(),
        }
    }
}