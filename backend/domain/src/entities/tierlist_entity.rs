use crate::entities::CardEntity;
use crate::entities::GradeEntity;

#[derive(Debug, Clone)]
pub struct TierlistEntity {
    pub id: String,
    pub name: String,
    pub is_public: bool,
    pub cover_image: String,
    pub author: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<GradeEntity>,
}