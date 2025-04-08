use crate::domain::entities::tierlist_entity::TierlistEntity;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::Error;
use crate::domain::entities::create_tierlist_entity::CreateTierlistEntity;
use crate::domain::mapper::TryEntityMapper;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TierlistModel {
    pub _id: ObjectId,
    pub name: String,
    pub author: ObjectId,
}

impl TryEntityMapper<TierlistEntity> for TierlistModel {
    fn to_entity(self) -> TierlistEntity {
        TierlistEntity {
            id: self._id.to_string(),
            name: self.name,
            author: self.author.to_string(),
        }
    }
}

impl TryFrom<TierlistEntity> for TierlistModel {
    type Error = Error;

    fn try_from(value: TierlistEntity) -> Result<Self, Self::Error> {
        let id = ObjectId::parse_str(value.id)?;

        let name = value.name;
        let author = ObjectId::parse_str(value.author)?;

        Ok(Self { _id: id, name, author })
    }
}

impl TryFrom<CreateTierlistEntity> for TierlistModel {
    type Error = Error;

    fn try_from(value: CreateTierlistEntity) -> Result<Self, Self::Error> {
        let id = ObjectId::new();

        let name = value.name;
        let author = ObjectId::parse_str(value.author)?;

        Ok(Self { _id: id, name, author })
    }
}