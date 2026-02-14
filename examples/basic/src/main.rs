use anyhow::anyhow;
use breach::{HttpError, http::StatusCode};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ForbiddenError {
    id: String,
}

#[derive(HttpError, Serialize)]
#[http(status = NOT_FOUND)]
#[serde(rename_all = "camelCase")]
struct NotFoundError {
    id: String,
}

#[derive(HttpError, Serialize)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum GetUserByIdError {
    #[http(status = FORBIDDEN)]
    Forbidden(ForbiddenError),

    NotFound(NotFoundError),

    #[http(status = INTERNAL_SERVER_ERROR)]
    Internal(#[serde(skip)] anyhow::Error),
}

#[derive(HttpError, Serialize)]
#[serde(
    tag = "code",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
enum UpdateUserError {
    GetUserById(GetUserByIdError),

    #[http(status = UNPROCESSABLE_ENTITY)]
    Validation,

    #[http(status = INTERNAL_SERVER_ERROR)]
    Internal(#[serde(skip)] anyhow::Error),
}

fn main() {
    let error = UpdateUserError::GetUserById(GetUserByIdError::Forbidden(ForbiddenError {
        id: "1".to_owned(),
    }));
    assert_eq!(StatusCode::FORBIDDEN, error.status());
    assert_eq!(
        json!({
            "code": "forbidden",
            "id": "1",
        }),
        serde_json::to_value(error).expect("serialized value")
    );

    let error = UpdateUserError::GetUserById(GetUserByIdError::NotFound(NotFoundError {
        id: "1".to_owned(),
    }));
    assert_eq!(StatusCode::NOT_FOUND, error.status());
    assert_eq!(
        json!({
            "code": "notFound",
            "id": "1",
        }),
        serde_json::to_value(error).expect("serialized value")
    );

    let error = UpdateUserError::Validation;
    assert_eq!(StatusCode::UNPROCESSABLE_ENTITY, error.status());
    assert_eq!(
        json!({
            "code": "validation",
        }),
        serde_json::to_value(error).expect("serialized value")
    );

    let error = UpdateUserError::GetUserById(GetUserByIdError::Internal(anyhow!("database error")));
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, error.status());
    assert_eq!(
        json!({
            "code": "internal",
        }),
        serde_json::to_value(error).expect("serialized value")
    );

    let error = UpdateUserError::Internal(anyhow!("database error"));
    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, error.status(),);
    assert_eq!(
        json!({
            "code": "internal",
        }),
        serde_json::to_value(error).expect("serialized value")
    );
}
