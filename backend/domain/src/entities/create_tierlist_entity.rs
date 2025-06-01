use crate::entities::CardEntity;
use crate::entities::GradeEntity;

#[derive(Debug, Clone)]
pub struct CreateTierlistEntity{
    pub name: String,
    pub is_public: bool,
    pub author: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<GradeEntity>,
}