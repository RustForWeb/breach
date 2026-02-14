use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use breach::HttpError;
use serde::Serialize;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    state::AppState,
    user::{
        errors::{CreateUserError, DeleteUserError, GetUserByIdError, UpdateUserError},
        schemas::{CreateUser, UpdateUser, User, UserPathParams},
        services::UserService,
    },
};

pub struct UserRoutes;

impl UserRoutes {
    pub fn router() -> OpenApiRouter<AppState> {
        OpenApiRouter::new()
            .routes(routes!(create_user))
            .routes(routes!(user, update_user, delete_user))
    }
}

#[derive(HttpError, Serialize)]
#[http(axum, utoipa)]
pub enum CreateUserRouteError {
    CreateUser(CreateUserError),
}

impl From<CreateUserError> for CreateUserRouteError {
    fn from(value: CreateUserError) -> Self {
        Self::CreateUser(value)
    }
}

#[utoipa::path(
    post,
    path = "/users/",
    operation_id = "createUser",
    summary = "Create user",
    description = "Create a user.",
    tags = ["User"],
    request_body = CreateUser,
    responses(
        (status = CREATED, description = "The user has been created.", body = User),
        CreateUserRouteError,
    )
)]
async fn create_user(
    State(AppState { database }): State<AppState>,
    Json(data): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), CreateUserRouteError> {
    let user = UserService::create(&database, data)?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[derive(HttpError, Serialize)]
#[http(axum, utoipa)]
pub enum GetUserRouteError {
    GetUserById(GetUserByIdError),
}

impl From<GetUserByIdError> for GetUserRouteError {
    fn from(value: GetUserByIdError) -> Self {
        Self::GetUserById(value)
    }
}

#[utoipa::path(
    get,
    path = "/users/{userId}",
    operation_id = "getUser",
    summary = "Get user",
    description = "Get a user.",
    tags = ["User"],
    params(UserPathParams),
    responses(
        (status = OK, description = "The user.", body = User),
        GetUserRouteError,
    )
)]
async fn user(
    State(AppState { database }): State<AppState>,
    Path(UserPathParams { user_id }): Path<UserPathParams>,
) -> Result<Json<User>, GetUserRouteError> {
    let user = UserService::by_id(&database, user_id)?;

    Ok(Json(user))
}

#[derive(HttpError, Serialize)]
#[http(axum, utoipa)]
pub enum UpdateUserRouteError {
    GetUserById(GetUserByIdError),

    UpdateUser(UpdateUserError),
}

impl From<GetUserByIdError> for UpdateUserRouteError {
    fn from(value: GetUserByIdError) -> Self {
        Self::GetUserById(value)
    }
}

impl From<UpdateUserError> for UpdateUserRouteError {
    fn from(value: UpdateUserError) -> Self {
        Self::UpdateUser(value)
    }
}

#[utoipa::path(
    patch,
    path = "/users/{userId}",
    operation_id = "updateUser",
    summary = "Update user",
    description = "Update a user.",
    tags = ["User"],
    params(UserPathParams),
    request_body = UpdateUser,
    responses(
        (status = OK, description = "The user has been updated.", body = User),
        UpdateUserRouteError,
    )
)]
async fn update_user(
    State(AppState { database }): State<AppState>,
    Path(UserPathParams { user_id }): Path<UserPathParams>,
    Json(data): Json<UpdateUser>,
) -> Result<Json<User>, UpdateUserRouteError> {
    let user = UserService::by_id(&database, user_id)?;

    let user = UserService::update(&database, user, data)?;

    Ok(Json(user))
}

#[derive(HttpError, Serialize)]
#[http(axum, utoipa)]
pub enum DeleteUserRouteError {
    GetUserById(GetUserByIdError),

    DeleteUser(DeleteUserError),
}

impl From<GetUserByIdError> for DeleteUserRouteError {
    fn from(value: GetUserByIdError) -> Self {
        Self::GetUserById(value)
    }
}

impl From<DeleteUserError> for DeleteUserRouteError {
    fn from(value: DeleteUserError) -> Self {
        Self::DeleteUser(value)
    }
}

#[utoipa::path(
    delete,
    path = "/users/{userId}",
    operation_id = "deleteUser",
    summary = "Delete user",
    description = "Delete a user.",
    tags = ["User"],
    params(UserPathParams),
    responses(
        (status = NO_CONTENT, description = "The user has been deleted."),
        DeleteUserRouteError,
    )
)]
async fn delete_user(
    State(AppState { database }): State<AppState>,
    Path(UserPathParams { user_id }): Path<UserPathParams>,
) -> Result<StatusCode, DeleteUserRouteError> {
    let user = UserService::by_id(&database, user_id)?;

    UserService::delete(&database, user)?;

    Ok(StatusCode::NO_CONTENT)
}
