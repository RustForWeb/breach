use breach::HttpError;
use serde::Serialize;
use utoipa::ToSchema;

use crate::error::NotFoundError;

#[derive(Serialize, ToSchema)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum UserValidationError {
    NameTooShort,
}

#[derive(HttpError, Serialize)]
#[http(utoipa)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum CreateUserError {
    #[http(status = UNPROCESSABLE_ENTITY)]
    Validation(UserValidationError),

    #[http(status = INTERNAL_SERVER_ERROR)]
    Internal(#[serde(skip)] anyhow::Error),
}

impl From<UserValidationError> for CreateUserError {
    fn from(value: UserValidationError) -> Self {
        Self::Validation(value)
    }
}

#[derive(HttpError, Serialize)]
#[http(utoipa)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum GetUserByIdError {
    NotFound(NotFoundError),

    #[http(status = INTERNAL_SERVER_ERROR)]
    Internal(#[serde(skip)] anyhow::Error),
}

#[derive(HttpError, Serialize)]
#[http(utoipa)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum UpdateUserError {
    #[http(status = UNPROCESSABLE_ENTITY)]
    Validation(UserValidationError),

    #[http(status = INTERNAL_SERVER_ERROR)]
    Internal(#[serde(skip)] anyhow::Error),
}

impl From<UserValidationError> for UpdateUserError {
    fn from(value: UserValidationError) -> Self {
        Self::Validation(value)
    }
}

#[derive(HttpError, Serialize)]
#[http(utoipa)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum DeleteUserError {
    #[http(status = INTERNAL_SERVER_ERROR)]
    Internal(#[serde(skip)] anyhow::Error),
}
