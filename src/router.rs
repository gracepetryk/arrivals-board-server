use axum::Router;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::{AppContext, controllers::*};

pub fn api(db: AppContext) -> (Router, OpenApi) {
    OpenApiRouter::new()
        .nest("/healthcheck", health::router().into())
        .nest("/flights", flights::router().into())
        .nest("/users", users::router())
        .nest("/auth", auth::router().into())
        .with_state(db.clone())
        .split_for_parts()
}
