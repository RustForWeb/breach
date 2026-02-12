use uuid::Uuid;

use crate::{
    database::Database,
    error::NotFoundError,
    user::{
        errors::{CreateUserError, DeleteUserError, GetUserByIdError, UpdateUserError},
        schemas::{CreateUser, UpdateUser, User},
    },
};

pub struct UserService;

impl UserService {
    pub fn by_id(database: &Database, id: Uuid) -> Result<User, GetUserByIdError> {
        database
            .user(id)
            .map_err(GetUserByIdError::Internal)?
            .ok_or_else(|| GetUserByIdError::NotFound(NotFoundError::new(id)))
    }

    pub fn create(database: &Database, data: CreateUser) -> Result<User, CreateUserError> {
        data.validate()?;

        let user = User {
            id: Uuid::now_v7(),
            name: data.name,
        };

        database
            .insert_user(user)
            .map_err(CreateUserError::Internal)
    }

    pub fn update(
        database: &Database,
        user: User,
        data: UpdateUser,
    ) -> Result<User, UpdateUserError> {
        data.validate()?;

        let mut user = user.clone();

        if let Some(name) = data.name {
            user.name = name;
        }

        database
            .insert_user(user)
            .map_err(UpdateUserError::Internal)
    }

    pub fn delete(database: &Database, user: User) -> Result<(), DeleteUserError> {
        database
            .remove_user(user)
            .map_err(DeleteUserError::Internal)
    }
}
