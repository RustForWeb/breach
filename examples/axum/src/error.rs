use breach::HttpError;
use serde::Serialize;
use uuid::Uuid;

#[derive(HttpError, Serialize)]
#[http(status = NOT_FOUND)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundError {
    id: Uuid,
}

impl NotFoundError {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
