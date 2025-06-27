use crate::models::{CardModel, TemplateGradeModel, TemplateModel};
use async_trait::async_trait;
use domain::entities::{CreateTemplateEntity, TemplateEntity, UpdateTemplateEntity};
use domain::error::ApiError;
use domain::mappers::TryEntityMapper;
use domain::repositories::AbstractTemplateRepository;
use domain::types::Pagination;
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::{Collection, Cursor, Database};

#[derive(Clone)]
pub struct TemplateRepository{
    collection: Collection<TemplateModel>
}

impl TemplateRepository {
    pub fn new(db: &Database) -> Self {
        let collection = db.collection("template");
        Self { collection }
    }
}

#[async_trait]
impl AbstractTemplateRepository for TemplateRepository {
    
    async fn get_template_by_id(&self, id: &str) -> Result<TemplateEntity, ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let template = find_template_by_id(&self.collection, id).await?;

        Ok(template.to_entity())
    }

    async fn get_template_of_user(&self, user_id: &str, can_see_private_templates: bool) -> Result<Vec<TemplateEntity>, ApiError> {
        let user_object_id = ObjectId::parse_str(user_id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let mut query= doc! { "author": user_object_id };
        if ! can_see_private_templates {
            query = doc! {"author": user_object_id, "is_public": true };
        }
        
        let cursor = self.collection.find(query)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

        let result = collect_cursor_to_list_of_template_entity(cursor).await;

        Ok(result)
    }

    async fn create_template(&self, template: CreateTemplateEntity) -> Result<String, ApiError> {
        let template = TemplateModel::try_from(template)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let result = self.collection.insert_one(template)
            .await
            .map_err(|err| ApiError::InternalError(format!("Failed to execute query: {err}")))?;
        
        let id = result.inserted_id.as_object_id()
            .ok_or(ApiError::InternalError("Failed to retrieve template id".to_owned()))?
            .to_string();

        Ok(id)
    }

    async fn update_template_by_id(&self, id: &str, template: UpdateTemplateEntity) -> Result<(), ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;
        
        let mut template_entity = find_template_by_id(&self.collection, id).await?;
        template_entity.name = template.name;
        template_entity.is_public = template.is_public;
        template_entity.tags = template.tags;
        template_entity.cards = template.cards.into_iter().map(CardModel::from).collect();
        template_entity.grades = template.grades.into_iter().map(TemplateGradeModel::from).collect();
        
        let query = doc! { "_id": id };
        self.collection.replace_one(query, template_entity)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute update: {e}")))?;

        Ok(())
    }

    async fn search(&self, search_title: &str, search_tags: Vec<&str>, pagination: Pagination) -> Result<Vec<TemplateEntity>, ApiError> {
        let query = build_query_for_full_search(search_title, search_tags);

        let cursor = self.collection.find(query)
            .skip((pagination.per_page as u64) * (pagination.page - 1))
            .limit(pagination.per_page as i64)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute search : {e}")))?;

        let result = collect_cursor_to_list_of_template_entity(cursor).await;

        Ok(result)
    }
}

async fn find_template_by_id(collection: &Collection<TemplateModel>, id: ObjectId) -> Result<TemplateModel, ApiError> {
    let template = collection.find_one(doc! { "_id": id })
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to execute query: {e}")))?;

    match template {
        None => Err(ApiError::NotFound(format!("template with id {id} not found"))),
        Some(template) => Ok(template),
    }
}

async fn collect_cursor_to_list_of_template_entity(cursor:  Cursor<TemplateModel>) -> Vec<TemplateEntity> {
    cursor.filter_map(|item| async {
            item.ok().map(|tl| tl.to_entity())
        })
        .collect()
        .await
}

fn build_query_for_full_search(title: &str, tags: Vec<&str>) -> Document {
    if tags.is_empty() {
        return doc! {
            "is_public": true,
            "name": doc! {
                "$regex": title,
                "$options": "i"
            },
        }
    }

    if title.is_empty() {
        return doc! {
            "is_public": true,
            "tags": doc! { "$all": tags }
        }
    }

    doc! {
        "is_public": true,
        "name": doc! {
            "$regex": title,
            "$options": "i"
        },
        "tags": doc! { "$all": tags }
    }
}