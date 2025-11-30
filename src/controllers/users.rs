use axum::{
    Json, debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use diesel::{
    insert_into,
    prelude::*,
    result::{DatabaseErrorKind, Error::DatabaseError},
};
use diesel_async::{RunQueryDsl, pooled_connection::bb8};
use serde::Deserialize;
use strum::Display;
use thiserror::Error;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    AppContext,
    errors::{ErrorReason, GetRequestError},
    models::User,
    schema::users,
};

pub fn router() -> OpenApiRouter<AppContext> {
    return OpenApiRouter::new()
        .routes(routes!(get_user, create_user))
        .routes(routes!(get_all_users));
}

#[utoipa::path(get, path = "/", responses((status = OK, body = Vec<User>)))]
async fn get_all_users(
    State(app_context): State<AppContext>,
) -> Result<Json<Vec<User>>, GetRequestError> {
    let mut conn = app_context.db.get().await?;

    let users: Vec<User> = users::table.load::<User>(&mut conn).await?;

    return Ok(Json(users));
}

#[utoipa::path(get, path = "/{id}", responses(
        (status = OK, body = User),
        (status = NOT_FOUND, body = ErrorReason<String>)
))]
async fn get_user(
    State(app_context): State<AppContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, GetRequestError> {
    let mut conn = app_context.db.get().await?;

    let user = users::table
        .select(User::as_select())
        .find(id)
        .first(&mut conn)
        .await?;

    return Ok(Json(user));
}

#[derive(Deserialize, Insertable, utoipa::ToSchema)]
#[diesel(table_name = users)]
struct CreateUser {
    pub email: String,
}

#[derive(Debug, Display, Error, utoipa::ToSchema)]
enum CreateUser4xxError {
    UserAlreadyExists,
    InvalidEmail,
}

#[derive(Debug, Error)]
#[error("An internal server error occured.")]
enum CreateUserError {
    DatabaseError(#[from] diesel::result::Error),
    ConnectionError(#[from] bb8::RunError),
    BadRequest(#[from] CreateUser4xxError),
}

impl IntoResponse for CreateUserError {
    fn into_response(self) -> axum::response::Response {
        eprintln!("{:#?}", self);
        let status = match &self {
            CreateUserError::DatabaseError(_) | CreateUserError::ConnectionError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            CreateUserError::BadRequest(_) => StatusCode::BAD_REQUEST,
        };

        ErrorReason::new(status.into(), self).into_response()
    }
}

#[utoipa::path(post, path = "/create", responses(
        (status = OK, body = User),
        (status = 400, body = ErrorReason<CreateUser4xxError>)
))]
#[debug_handler]
async fn create_user(
    State(app_context): State<AppContext>,
    Json(user): Json<CreateUser>,
) -> Result<Json<User>, CreateUserError> {
    let mut conn = app_context.db.get().await?;

    let user = insert_into(users::table)
        .values(&vec![user])
        .returning(users::all_columns)
        .get_result::<User>(&mut conn)
        .await;

    match user {
        Ok(user) => Ok(user.into()),
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
            Err(CreateUser4xxError::UserAlreadyExists.into())
        }
        Err(e) => Err(e.into()),
    }
}
