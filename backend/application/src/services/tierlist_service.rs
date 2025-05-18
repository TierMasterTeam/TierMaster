use derive_new::new;
use domain::entities::{CreateTierlistEntity, TierlistEntity, UpdateTierlistEntity};
use domain::error::ApiError;
use domain::repositories::AbstractTierlistRepository;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(new)]
pub struct TierlistService{
    repo: Arc<dyn AbstractTierlistRepository>,
}

impl TierlistService {
    pub async fn get_all_tierlists(&self) -> Result<Vec<TierlistEntity>, ApiError> {
        self.repo.get_all_tierlists().await
    }

    pub async fn get_tierlist_by_id(&self, id: &str) -> Result<TierlistEntity, ApiError> {
        self.repo.get_tierlist_by_id(id).await
    }

    pub async fn get_tierlists_of_user(&self, user_id: &str) -> Result<Vec<TierlistEntity>, ApiError> {
        self.repo.get_tierlist_of_user(user_id).await
    }

    pub async fn create_tierlist(&self, tierlist: CreateTierlistEntity) -> Result<(), ApiError> {
        self.repo.create_tierlist(tierlist).await
    }

    pub async fn update_tierlist_by_id(&self, id: &str, tierlist: UpdateTierlistEntity) -> Result<(), ApiError>  {
        self.repo.update_tierlist_by_id(id, tierlist).await
    }

    pub async fn search(&self, search_text: &str) -> Result<Vec<TierlistEntity>, ApiError> {
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);
        self.repo.search(title, tags).await
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
    use crate::services::tierlist_service::extract_title_and_tags_from_search_text;

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
        let search_text = " #test #rust #tierlist ";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"tierlist"));
    }

    #[test]
    fn when_title_first_and_tags_then_title_and_tags_are_correct() {
        let search_text = "Test title #test #rust #tierlist";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"tierlist"));
    }

    #[test]
    fn when_title_last_and_tags_then_title_and_tags_are_correct() {
        let search_text = "#test #rust #tierlist Test title";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"tierlist"));
    }

    #[test]
    fn when_title_middle_and_tags_then_title_and_tags_are_correct() {
        let search_text = "#test #rust Test title #tierlist";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"tierlist"));
    }

    #[test]
    fn when_search_text_contains_duplicate_tag_then_tags_contains_no_duplicate() {
        let search_text = "#test #rust #rust Test title #tierlist";
        let (title, tags) = extract_title_and_tags_from_search_text(search_text);

        assert_eq!(title, "Test title");
        assert_eq!(tags.len(), 3);
        assert!(tags.contains(&"test"));
        assert!(tags.contains(&"rust"));
        assert!(tags.contains(&"tierlist"));
    }
}
