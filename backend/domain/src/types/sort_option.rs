use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SortOption {
    pub field: String,
    pub asc: bool,
}