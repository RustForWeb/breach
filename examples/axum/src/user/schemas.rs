use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::user::errors::UserValidationError;

#[derive(Deserialize, IntoParams)]
pub struct UserPathParams {
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateUser {
    pub name: String,
}

impl CreateUser {
    pub fn validate(&self) -> Result<(), UserValidationError> {
        if self.name.is_empty() {
            return Err(UserValidationError::NameTooShort);
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateUser {
    pub name: Option<String>,
}

impl UpdateUser {
    pub fn validate(&self) -> Result<(), UserValidationError> {
        if let Some(name) = &self.name
            && name.is_empty()
        {
            return Err(UserValidationError::NameTooShort);
        }

        Ok(())
    }
}
