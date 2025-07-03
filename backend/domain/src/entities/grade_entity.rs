use crate::entities::{CardEntity, GradeRoomEntity};

#[derive(Clone, Debug)]
pub struct GradeEntity {
    pub name: String,
    pub color: String,
    pub cards: Vec<CardEntity>
}

impl From<GradeRoomEntity> for GradeEntity {
    fn from(value: GradeRoomEntity) -> Self {
        Self { 
            name: value.name,
            color: value.color,
            cards: value.cards.into_iter().map(Into::into).collect()
        }
    }
}