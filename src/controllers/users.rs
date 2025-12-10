use std::sync::LazyLock;

use axum::{
    Json, debug_handler,
    extract::{Path, State},
};
use diesel::{
    insert_into,
    prelude::*,
    result::{DatabaseErrorKind, Error::DatabaseError},
};
use diesel_async::RunQueryDsl;
use http::StatusCode;
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    AppContext, errors::error_enum, models::User, schema::users,
};

pub fn router() -> OpenApiRouter<AppContext> {
    return OpenApiRouter::new().routes(routes!(get_user, create_user));
}

error_enum! { GetUserError {
    #[response(status = StatusCode::NOT_FOUND, description = "User Not Found")]
    NotFound
}}

#[utoipa::path(get, path = "/{id}", responses(
        (status = OK, body = User),
        GetUserError
))]
async fn get_user(
    State(app_context): State<AppContext>,
    Path(id): Path<uuid::Uuid>,
) -> Result<Json<User>, GetUserError> {
    let mut conn = app_context.db.get().await?;

    let user = users::table
        .select(User::as_select())
        .find(id)
        .first(&mut conn)
        .await
        .map_err(|err| match err {
            diesel::result::Error::NotFound => GetUserError::NotFound,
            _ => err.into(),
        })?;

    Ok(Json(user))
}

#[derive(Deserialize, Insertable, utoipa::ToSchema)]
#[diesel(table_name = users)]
struct CreateUser {
    pub email: String,
}

#[derive(Debug, ToSchema, Serialize)]
enum CreateUserBadRequest {
    InvalidEmail,
}

error_enum! { CreateUserError {
    #[response(status = StatusCode::CONFLICT, description = "User Already Exists")]
    UserAlreadyExists,

    #[response(status = StatusCode::BAD_REQUEST, description = "Bad Request")]
    BadRequest(CreateUserBadRequest),
}}

#[utoipa::path(post, path = "/create", responses((status=200, response=User), CreateUserError))]
#[debug_handler]
async fn create_user(
    State(app_context): State<AppContext>,
    Json(user): Json<CreateUser>,
) -> Result<Json<User>, CreateUserError> {
    if !is_valid_email(&user.email) {
        return Err(CreateUserError::BadRequest(
            CreateUserBadRequest::InvalidEmail.into(),
        ));
    }

    let mut conn = app_context.db.get().await?;

    let user = insert_into(users::table)
        .values(&vec![user])
        .returning(users::all_columns)
        .get_result::<User>(&mut conn)
        .await;

    match user {
        Ok(user) => Ok(Json(user)),
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err(CreateUserError::UserAlreadyExists)
        }
        Err(e) => Err(e.into()),
    }
}

static EMAIL_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^@]+@[^@]+\.[^@]+$").unwrap());

fn is_valid_email<'a>(email: &'a str) -> bool {
    EMAIL_REGEX.is_match(email)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("me@gracepetryk.com"))
    }

    #[test]
    fn test_is_valid_email_rejects_invalid() {
        assert!(!is_valid_email("not_an_email"));
        assert!(!is_valid_email("@@@"));
        assert!(!is_valid_email("me@foo@foo@com"));
    }
}
