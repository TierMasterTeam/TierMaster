use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchQueryPresenter {
    pub(crate) query: String,
}