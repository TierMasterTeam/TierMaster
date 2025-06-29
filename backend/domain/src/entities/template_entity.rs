use crate::entities::{CardEntity, TemplateGradeEntity};

#[derive(Debug, Clone)]
pub struct TemplateEntity {
    pub id: String,
    pub name: String,
    pub is_public: bool,
    pub author: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<TemplateGradeEntity>,
}