use crate::database::Database;

#[derive(Clone, Default)]
pub struct AppState {
    pub database: Database,
}
