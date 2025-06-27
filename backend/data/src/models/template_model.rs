use crate::models::{CardModel, TemplateGradeModel};
use domain::entities::{CreateTemplateEntity, TemplateEntity};
use domain::mappers::{EntityMapper, TryEntityMapper};
use mongodb::bson::oid::{Error, ObjectId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub is_public: bool,
    pub author: ObjectId,
    pub tags: Vec<String>,
    pub cards: Vec<CardModel>,
    pub grades: Vec<TemplateGradeModel>,
}

impl TryEntityMapper<TemplateEntity> for TemplateModel {
    fn to_entity(self) -> TemplateEntity {
        TemplateEntity {
            id: self.id.unwrap_or_default().to_string(),
            is_public: self.is_public,
            name: self.name,
            author: self.author.to_string(),
            tags: self.tags,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
        }
    }
}

impl TryFrom<TemplateEntity> for TemplateModel {
    type Error = Error;

    fn try_from(value: TemplateEntity) -> Result<Self, Self::Error> {
        let id = ObjectId::parse_str(value.id)?;

        Ok(Self {
            id: Some(id),
            name: value.name,
            is_public: value.is_public,
            author: ObjectId::parse_str(value.author)?,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        })
    }
}

impl TryFrom<CreateTemplateEntity> for TemplateModel {
    type Error = Error;

    fn try_from(value: CreateTemplateEntity) -> Result<Self, Self::Error> {
        Ok(Self {
            id: None,
            name: value.name,
            is_public: value.is_public,
            author: ObjectId::parse_str(value.author)?,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
        })
    }
}