use serde::{Deserialize, Serialize};
use domain::entities::{CreateTierlistEntity, TierlistEntity};
use domain::mappers::{TryEntityMapper, EntityMapper};
use mongodb::bson::oid::{ObjectId, Error};
use crate::models::{CardModel, GradeModel};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TierlistModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub author: ObjectId,
    pub cards: Vec<CardModel>,
    pub grades: Vec<GradeModel>,
}

impl TryEntityMapper<TierlistEntity> for TierlistModel {
    fn to_entity(self) -> TierlistEntity {
        TierlistEntity {
            id: self.id.unwrap_or_default().to_string(),
            name: self.name,
            author: self.author.to_string(),
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl TryFrom<TierlistEntity> for TierlistModel {
    type Error = Error;

    fn try_from(value: TierlistEntity) -> Result<Self, Self::Error> {
        let id = ObjectId::parse_str(value.id)?;

        Ok(Self {
            id: Some(id),
            name: value.name,
            author: ObjectId::parse_str(value.author)?,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        })
    }
}

impl TryFrom<CreateTierlistEntity> for TierlistModel {
    type Error = Error;

    fn try_from(value: CreateTierlistEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: None,
            name: value.name,
            author: ObjectId::parse_str(value.author)?,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        })
    }
}