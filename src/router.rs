use std::{env, path::PathBuf};

use axum::{Json, Router, routing::get};
use tower_http::services::ServeFile;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_rapidoc::RapiDoc;

use crate::{
    AppContext, AppError,
    controllers::{auth, flights, health, users},
};

pub fn api(db: AppContext) -> Router {
    let (router, openapi) = OpenApiRouter::new()
        .nest("/healthcheck", health::router().into())
        .nest("/flights", flights::router().into())
        .nest("/users", users::router())
        .nest("/auth", auth::router().into())
        .with_state(db.clone())
        .split_for_parts();

    router
        .route("/openapi.json", get(Json(openapi)))
        .nest("/docs/api", RapiDoc::new("/openapi.json").into())
}

pub fn api_docs(openapi_url: String) -> Router {
    RapiDoc::new(openapi_url).into()
}

pub fn crate_docs() -> Router {
    let package_name = env!("CARGO_CRATE_NAME");
    let crate_docs_path = PathBuf::from(
        env::var("CRATE_DOCS_LOCATION").unwrap_or(format!("target/doc/{package_name}/index.html")),
    );

    Router::new().route_service("/crate-docs", ServeFile::new(crate_docs_path))
}
