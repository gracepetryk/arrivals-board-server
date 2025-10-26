use axum::{routing::get, Router};

pub fn router() -> Router {
    return Router::new()
        .route("/", get(healthcheck))
}

async fn healthcheck() -> String {
    "200 ok".to_string()
}
