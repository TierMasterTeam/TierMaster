use std::sync::OnceLock;
use derive_new::new;
use mongodb::Database;

/// App state singleton.
/// Retrieve the state with
/// ```
///     use backend::core::app_state::{APP_STATE};
///     let state = APP_STATE.get();
/// ```
/// See [AppState] for more information.
pub static APP_STATE: OnceLock<AppState> = OnceLock::new();

/// An application state containing information such as databases connections.
#[derive(new, Debug)]
pub struct AppState {
    mongodb: Database,
}
impl AppState {
    pub fn mongodb(&self) -> &Database {
        &self.mongodb
    }
}