use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQueryPresenter {
    pub query: String,
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(alias="per-page", default = "default_per_page")]
    pub per_page: u8,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u8 {
    25
}