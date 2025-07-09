use crate::models::{CardModel, TemplateGradeModel, TemplateModel};
use async_trait::async_trait;
use domain::entities::{CreateTemplateEntity, TemplateEntity, UpdateTemplateEntity};
use domain::error::ApiError;
use domain::mappers::TryEntityMapper;
use domain::repositories::AbstractTemplateRepository;
use domain::types::{Pagination, SortOption};
use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::{Collection, Cursor, Database};

static SUPPORTED_SORT_FIELDS: &'static [&str] = &[
    "created_at",
];

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
    
    async fn get_template_by_id(&self, id: &str, user_id: Option<String>) -> Result<TemplateEntity, ApiError> {
        let id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;

        let template = find_template_by_id(&self.collection, id).await?;
        
        let template_is_private = ! template.is_public;
        let user_is_not_author = match &user_id {
            Some(uid) => template.author.to_string() != *uid,
            None => true,
        };
        
        if template_is_private && user_is_not_author {
            return Err(ApiError::Forbidden("You do not have permission to access this template because it is private.".to_string()))
        }

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
        template_entity.cover_image = template.cover_image;
        template_entity.cards = template.cards.into_iter().map(CardModel::from).collect();
        template_entity.grades = template.grades.into_iter().map(TemplateGradeModel::from).collect();
        
        let query = doc! { "_id": id };
        self.collection.replace_one(query, template_entity)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute update: {e}")))?;

        Ok(())
    }

    async fn search(&self, search_title: &str, search_tags: Vec<&str>, pagination: Pagination, sort_option: Option<SortOption>) -> Result<Vec<TemplateEntity>, ApiError> {
        let query = build_query_for_full_search(search_title, search_tags);

        let mut query_builder = self.collection.find(query)
            .skip((pagination.per_page as u64) * (pagination.page - 1))
            .limit(pagination.per_page as i64);

        let sort = build_sort(sort_option)?;
        if sort.is_some() {
            query_builder = query_builder.sort(sort.unwrap());
        }

        let cursor = query_builder
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute search : {e}")))?;

        let result = collect_cursor_to_list_of_template_entity(cursor).await;

        Ok(result)
    }
    
    async fn delete_by_id(&self, id: &str) -> Result<(), ApiError> {
        let template_id = ObjectId::parse_str(id)
            .map_err(|err| ApiError::BadRequest(err.to_string()))?;
        
        let query = doc! { "_id": template_id };
        self.collection.delete_one(query)
            .await
            .map_err(|e| ApiError::InternalError(format!("Failed to execute delete : {e}")))?;
        
        Ok(())
    }
}

fn build_sort(sort_option: Option<SortOption>) -> Result<Option<Document>, ApiError> {
    if sort_option.is_none() {
        return Ok(None);
    }

    let sort = sort_option.unwrap().clone();
    let sort_field = sort.field;

    if ! SUPPORTED_SORT_FIELDS.contains(&sort_field.as_str()) {
        return Err(ApiError::BadRequest(format!("Templates can not be sorted by '{sort_field}'")));
    }

    Ok(Some(doc! {
        sort_field: if sort.asc { 1 } else { -1 },
    }))
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
    if tags.is_empty() && title.is_empty() {
        return doc! {
            "is_public": true,
        }
    }

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