use serde::{Deserialize, Serialize};
use domain::entities::CardEntity;
use domain::mappers::EntityMapper;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CardModel {
    pub name: String,
    pub image: String,
}

impl From<CardEntity> for CardModel {
    fn from(value: CardEntity) -> Self {
        Self {
            name: value.name,
            image: value.image,
        }
    }
}
impl EntityMapper<CardEntity> for CardModel {
    fn to_entity(self) -> CardEntity {
        CardEntity {
            name: self.name,
            image: self.image,
        }
    }
}