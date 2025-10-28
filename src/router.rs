use axum::Router;

use crate::{
    AppContext,
    controllers::{flights, health, users},
};

pub fn route(db: AppContext) -> Router {
    Router::new()
        .nest("/api/healthcheck", health::router())
        .nest("/api/flights", flights::router())
        .nest("/api/users", users::router())
        .with_state(db.clone())
}
