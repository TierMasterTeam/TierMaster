use serde::{Deserialize, Serialize};
use domain::entities::GradeEntity;
use domain::mappers::EntityMapper;
use crate::presenters::CardPresenter;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GradePresenter {
    pub name: String,
    pub color: String,
    pub cards: Vec<CardPresenter>
}

impl From<GradeEntity> for GradePresenter {
    fn from(value: GradeEntity) -> Self {
        Self {
            name: value.name,
            color: value.color,
            cards: value.cards.into_iter().map(Into::into).collect(),
        }
    }
}

impl EntityMapper<GradeEntity> for GradePresenter {
    fn to_entity(self) -> GradeEntity {
        GradeEntity {
            name: self.name,
            color: self.color,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}