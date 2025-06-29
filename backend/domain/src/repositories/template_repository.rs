use crate::entities::{CreateTemplateEntity, TemplateEntity, UpdateTemplateEntity};
use crate::error::ApiError;
use crate::types::Pagination;
use async_trait::async_trait;

#[async_trait]
pub trait AbstractTemplateRepository: Send + Sync {
    async fn get_template_by_id(&self, id: &str, user_id: Option<String>) -> Result<TemplateEntity, ApiError>;
    async fn get_template_of_user(&self, user_id: &str, can_see_private_templates: bool) -> Result<Vec<TemplateEntity>, ApiError>;

    async fn create_template(&self, template: CreateTemplateEntity) -> Result<String, ApiError>;

    async fn update_template_by_id(&self, id: &str, template: UpdateTemplateEntity) -> Result<(), ApiError>;
    async fn search(&self, search_title: &str, search_tags: Vec<&str>, pagination: Pagination) -> Result<Vec<TemplateEntity>, ApiError>;
}