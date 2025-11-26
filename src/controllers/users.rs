use anyhow::Context;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    routing::post,
};
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use serde::{Deserialize, Serialize};
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{AppContext, AppError, models::User, schema::users};

pub fn router() -> OpenApiRouter<AppContext> {
    return OpenApiRouter::new()
        .routes(routes!(get_user))
        .routes(routes!(get_all_users))
        .route("/create", post(create_user));
}

#[derive(Serialize, utoipa::ToSchema)]
struct UserResult {
    #[serde(flatten)]
    user: User,
}

impl UserResult {
    fn new(user: User) -> Self {
        UserResult { user: user }
    }
}

#[utoipa::path(get, path = "/", responses((status = OK, body = Vec<User>)))]
async fn get_all_users(State(app_context): State<AppContext>) -> Result<Json<Vec<User>>, AppError> {
    let mut conn = app_context.db.get().await?;

    let users: Vec<User> = users::table.load::<User>(&mut conn).await?;

    return Ok(Json(users));
}

#[utoipa::path(get, path = "/{id}", responses((status = OK, body = UserResult)))]
async fn get_user(
    State(app_context): State<AppContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResult>, AppError> {
    let mut conn = app_context.db.get().await?;

    let user = users::table
        .select(User::as_select())
        .find(id)
        .first(&mut conn)
        .await
        .context(StatusCode::NOT_FOUND)?;

    return Ok(Json(UserResult::new(user)));
}

#[derive(Deserialize, Insertable, utoipa::ToSchema)]
#[diesel(table_name = users)]
struct CreateUser {
    pub email: String,
}

async fn create_user(
    State(app_context): State<AppContext>,
    Json(user): Json<CreateUser>,
) -> Result<Json<UserResult>, AppError> {
    let mut conn = app_context.db.get().await?;

    let query_result = insert_into(users::table)
        .values(&vec![user])
        .returning(users::all_columns)
        .get_result::<User>(&mut conn)
        .await;

    todo!()
}
