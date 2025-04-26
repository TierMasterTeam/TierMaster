use serde::{Deserialize, Serialize};
use domain::entities::CardEntity;
use domain::mappers::EntityMapper;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPresenter {
    pub name: String,
    pub image: String,
}

impl From<CardEntity> for CardPresenter {
    fn from(value: CardEntity) -> Self {
        Self {
            name: value.name,
            image: value.image,
        }
    }
}

impl EntityMapper<CardEntity> for CardPresenter {
    fn to_entity(self) -> CardEntity {
        CardEntity {
            name: self.name,
            image: self.image,
        }
    }
}