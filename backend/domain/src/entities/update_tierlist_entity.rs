use crate::entities::{CardEntity, GradeEntity, TierlistRoomEntity};

#[derive(Debug, Clone)]
pub struct UpdateTierlistEntity {
    pub name: String,
    pub is_public: bool,
    pub tags: Vec<String>,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<GradeEntity>,
}

impl From<TierlistRoomEntity> for  UpdateTierlistEntity {
    fn from(value: TierlistRoomEntity) -> Self {
        Self { 
            name: value.name, 
            is_public: value.is_public,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect()
        }
    }
}