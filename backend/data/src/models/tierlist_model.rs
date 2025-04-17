use serde::{Deserialize, Serialize};
use domain::entities::{CreateTierlistEntity, TierlistEntity};
use domain::mappers::TryEntityMapper;
use mongodb::bson::oid::{ObjectId, Error};

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