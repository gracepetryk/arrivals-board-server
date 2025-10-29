use anyhow::Context;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    AppContext,
    models::{TrackingParams, User},
    schema::users,
};

pub fn router() -> Router<AppContext> {
    return Router::new().route("/{username}", get(get_user));
}

#[derive(Serialize)]
struct UserResultInner {
    #[serde(flatten)]
    user: User,

    #[serde(flatten)]
    tracking_params: Vec<TrackingParams>,
}

#[derive(Serialize)]
struct UserResult {
    user: UserResultInner,
}

async fn get_user(
    State(app_context): State<AppContext>,
    Path(id): Path<Uuid>,
) -> crate::Result<Json<UserResult>> {
    let mut conn = app_context.db.get().await?;

    let user = users::table
        .select(User::as_select())
        .find(id)
        .first(&mut conn)
        .await
        .context(StatusCode::NOT_FOUND)?;

    let tracking_params = TrackingParams::belonging_to(&user)
        .select(TrackingParams::as_select())
        .load(&mut conn)
        .await?;

    return Ok(Json(UserResult {
        user: UserResultInner {
            user: user,
            tracking_params: tracking_params,
        },
    }));
}
