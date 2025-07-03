use crate::entities::{CardEntity, TemplateGradeEntity};

#[derive(Debug, Clone)]
pub struct CreateTemplateEntity {
    pub name: String,
    pub is_public: bool,
    pub author: String,
    pub cover_image: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<TemplateGradeEntity>,
}