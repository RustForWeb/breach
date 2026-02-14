use breach::HttpError;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(HttpError, Serialize, ToSchema)]
#[http(status = NOT_FOUND, utoipa)]
#[serde(rename_all = "camelCase")]
pub struct NotFoundError {
    id: Uuid,
}

impl NotFoundError {
    pub fn new(id: Uuid) -> Self {
        Self { id }
    }
}
