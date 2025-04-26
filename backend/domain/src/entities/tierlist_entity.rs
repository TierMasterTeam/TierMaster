use crate::entities::CardEntity;
use crate::entities::GradeEntity;

#[derive(Debug, Clone)]
pub struct TierlistEntity {
    pub id: String,
    pub name: String,
    pub author: String,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<GradeEntity>,
}