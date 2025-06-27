use domain::entities::TemplateGradeEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TemplateGradePresenter {
    pub name: String,
    pub color: String,
}

impl From<TemplateGradeEntity> for TemplateGradePresenter {
    fn from(value: TemplateGradeEntity) -> Self {
        Self {
            name: value.name,
            color: value.color,
        }
    }
}

impl EntityMapper<TemplateGradeEntity> for TemplateGradePresenter {
    fn to_entity(self) -> TemplateGradeEntity {
        TemplateGradeEntity {
            name: self.name,
            color: self.color,
        }
    }
}