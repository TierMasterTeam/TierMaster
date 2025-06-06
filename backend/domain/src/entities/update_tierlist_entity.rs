use crate::entities::{CardEntity, GradeEntity};

#[derive(Debug, Clone)]
pub struct UpdateTierlistEntity {
    pub name: String,
    pub is_public: bool,
    pub tags: Vec<String>,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<GradeEntity>,
}