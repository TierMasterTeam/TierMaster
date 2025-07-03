use crate::entities::{CardEntity, TemplateGradeEntity};
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct TemplateEntity {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub is_public: bool,
    pub author: String,
    pub cover_image: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardEntity>,
    pub grades: Vec<TemplateGradeEntity>,
}