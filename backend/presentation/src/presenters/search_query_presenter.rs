use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQueryPresenter {
    #[serde(default = "default_query")]
    pub query: String,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(alias="per-page", default = "default_per_page")]
    pub per_page: u8,
    
    #[serde(alias="sort-by")]
    pub sort_by: Option<String>,
    #[serde(alias="sort-asc", default = "default_sort_asc")]
    pub sort_asc: bool,
}

fn default_query() -> String {
    "".to_string()
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u8 {
    25
}

fn default_sort_asc() -> bool {
    false
}