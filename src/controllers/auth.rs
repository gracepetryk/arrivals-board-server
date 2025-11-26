use axum::{Router, routing::post};

use crate::AppContext;

pub fn router() -> Router<AppContext> {
    return Router::new().route("/signin", post(signin));
}

async fn signin() {}
