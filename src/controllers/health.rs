use axum::{Router, routing::get};

use crate::AppContext;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/", get(healthcheck));
}

async fn healthcheck() -> String {
    "200 ok".to_string()
}
