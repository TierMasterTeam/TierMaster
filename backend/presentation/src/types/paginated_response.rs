use serde::Serialize;

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub page: u64,
    #[serde(rename = "perPage")]
    pub per_page: u8,
    pub data: Vec<T>,
}
