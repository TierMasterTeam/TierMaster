use serde::{Deserialize, Serialize};
use domain::entities::GradeEntity;
use domain::mappers::EntityMapper;
use crate::models::CardModel;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GradeModel {
    pub name: String,
    pub color: String,
    pub cards: Vec<CardModel>
}

impl From<GradeEntity> for GradeModel {
    fn from(value: GradeEntity) -> Self {
        Self {
            name: value.name,
            color: value.color,
            cards: value.cards.into_iter().map(Into::into).collect()
        }
    }
}
impl EntityMapper<GradeEntity> for GradeModel {
    fn to_entity(self) -> GradeEntity {
        GradeEntity {
            name: self.name,
            color: self.color,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}