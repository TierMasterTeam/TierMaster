use derive_new::new;
use domain::entities::{CreateTemplateEntity, TemplateEntity, UpdateTemplateEntity};
use domain::error::ApiError;
use domain::repositories::AbstractTemplateRepository;
use domain::types::{Pagination, SortOption};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(new, Clone)]
pub struct TemplateService {
    repo: Arc<dyn AbstractTemplateRepository>,
}

impl TemplateService {
    pub async fn get_template_by_id(&self, id: &str, user_id: Option<String>) -> Result<TemplateEntity, ApiError> {
        self.repo.get_template_by_id(id, user_id).await
    }

    pub async fn get_templates_of_user(&self, user_id: &str, can_see_private_templates: bool) -> Result<Vec<TemplateEntity>, ApiError> {
        self.repo.get_template_of_user(user_id, can_see_private_templates).await
    }

    pub async fn create_template(&self, template: CreateTemplateEntity) -> Result<String, ApiError> {
        self.repo.create_template(template).await
    }

    pub async fn update_template_by_id(&self, id: &str, template: UpdateTemplateEntity) -> Result<(), ApiError>  {
        self.repo.update_template_by_id(id, template).await
    }

    pub async fn search(&self, search_text: &str, pagination: Pagination, sort_option: Option<SortOption>) -> Result<Vec<TemplateEntity>, ApiError> {
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);
        self.repo.search(title, tags, pagination, sort_option).await
    }
    
    pub async fn delete_by_id(&self, id: &str) -> Result<(), ApiError> {
        self.repo.delete_by_id(id).await
    }
}


fn extract_title_and_tags_from_search_text(search_text: &str) -> (&str, Vec<&str>) {
    let has_no_tags = !search_text.contains("#");
    if has_no_tags {
        return (search_text.trim(), Vec::new());
    }

    let elements = search_text.split("#").collect::<Vec<&str>>();
    let mut tags_set = HashSet::new();
    let mut title = "";

    for (index, element) in elements.iter().enumerate() {
        let trimmed_element = element.trim();

        if trimmed_element.is_empty() {
            continue;
        }

        // if the first element is not empty then it can't be a tag because of the split on #
        if index == 0 {
            title = trimmed_element;
            continue;
        }

        // if the element has spaces then there is the title, tags can't have multiple words
        if trimmed_element.contains(' ') {
            let words = trimmed_element.split_whitespace().collect::<Vec<&str>>();
            if let Some(first_word) = words.first() {
                tags_set.insert(*first_word);

                if title.is_empty() {
                    let rest = trimmed_element[first_word.len()..].trim_start();
                    title = rest;
                }
            }

            continue;
        }

        tags_set.insert(trimmed_element);
    }

    (title, tags_set.into_iter().collect())
}


#[cfg(test)]
mod extract_title_and_tags_from_search_text_tests {
    use crate::services::template_service::extract_title_and_tags_from_search_text;

    #[test]
    fn when_search_text_is_empty_then_title_and_tags_are_empty() {
        let search_text = "";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "");
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn when_no_tags_then_title_is_correct_and_tags_is_empty() {
        let search_text = " Test title ";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 0);
    }

    #[test]
    fn when_no_title_then_title_is_empty_and_tags_are_correct() {
        let search_text = " #test #rust #template ";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"template"));
    }

    #[test]
    fn when_title_first_and_tags_then_title_and_tags_are_correct() {
        let search_text = "Test title #test #rust #template";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"template"));
    }

    #[test]
    fn when_title_last_and_tags_then_title_and_tags_are_correct() {
        let search_text = "#test #rust #template Test title";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"template"));
    }

    #[test]
    fn when_title_middle_and_tags_then_title_and_tags_are_correct() {
        let search_text = "#test #rust Test title #template";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"template"));
    }

    #[test]
    fn when_search_text_contains_duplicate_tag_then_tags_contains_no_duplicate() {
        let search_text = "#test #rust #rust Test title #template";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"template"));
    }
}
