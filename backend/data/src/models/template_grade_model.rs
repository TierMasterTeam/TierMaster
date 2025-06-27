use domain::entities::TemplateGradeEntity;
use domain::mappers::EntityMapper;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TemplateGradeModel {
    pub name: String,
    pub color: String
}

impl From<TemplateGradeEntity> for TemplateGradeModel {
    fn from(value: TemplateGradeEntity) -> Self {
        Self {
            name: value.name,
            color: value.color,
        }
    }
}
impl EntityMapper<TemplateGradeEntity> for TemplateGradeModel {
    fn to_entity(self) -> TemplateGradeEntity {
        TemplateGradeEntity {
            name: self.name,
            color: self.color,
        }
    }
}