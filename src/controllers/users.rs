use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use uuid::Uuid;

use crate::{
    AppContext, AppError,
    models::user::{TrackingParams, User},
    schema::{tracking_params, users},
};

pub fn router() -> Router<AppContext> {
    return Router::new().route("/{username}", get(get_user));
}

struct UserResult {
    user: User,
    tracking_params: Vec<TrackingParams>,
}
async fn get_user(
    State(app_context): State<AppContext>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserResult>, AppError> {
    let conn = &mut app_context.db.get().await?;
    let user = users::table
        .select(User::as_select())
        .find(id)
        .first(conn)
        .await?;

    return Ok(Json(user));
}
