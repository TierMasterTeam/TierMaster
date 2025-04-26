use crate::entities::CardEntity;

#[derive(Clone, Debug)]
pub struct GradeEntity {
    pub name: String,
    pub color: String,
    pub cards: Vec<CardEntity>
}