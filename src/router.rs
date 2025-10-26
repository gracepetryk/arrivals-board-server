use axum::Router;

use crate::controllers::{flights, health, users};


pub fn route() -> Router {
    Router::new()
        .nest("/api/healthcheck", health::router())
        .nest("/api/flights", flights::router())
        .nest("/api/users", users::router())
}
