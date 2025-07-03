use crate::presenters::{CardPresenter, TemplateGradePresenter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use domain::entities::CreateTemplateEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTemplatePresenter {
    pub name: String,
    pub author: String,
    pub is_public: bool,
    #[serde(rename = "coverImage")]
    pub cover_image: String,
    pub tags: Vec<String>,
    pub cards: Vec<CardPresenter>,
    pub grades: Vec<TemplateGradePresenter>
}

impl EntityMapper<CreateTemplateEntity> for CreateTemplatePresenter {
    fn to_entity(self) -> CreateTemplateEntity {
        CreateTemplateEntity {
            name: self.name,
            is_public: self.is_public,
            author: self.author,
            tags: self.tags,
            cards: self.cards.into_iter().map(EntityMapper::to_entity).collect(),
            grades: self.grades.into_iter().map(EntityMapper::to_entity).collect(),
            cover_image: self.cover_image,
        }
    }
}

impl From<CreateTemplateEntity> for CreateTemplatePresenter {
    fn from(value: CreateTemplateEntity) -> Self {
        Self {
            name: value.name,
            author: value.author,
            is_public: value.is_public,
            tags: value.tags,
            cards: value.cards.into_iter().map(Into::into).collect(),
            grades: value.grades.into_iter().map(Into::into).collect(),
            cover_image: value.cover_image,
        }
    }
}

impl IntoResponse for CreateTemplatePresenter {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}