use std::fmt::Debug;

use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::PoolError;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, bb8::Pool};
use utoipa_rapidoc::RapiDoc;

mod controllers;
mod errors;
mod models;
mod router;
mod schema;

#[derive(Clone, Debug)]
struct AppContext {
    db: Pool<AsyncPgConnection>,
}

async fn get_connection_pool(connection_url: String) -> Result<Pool<AsyncPgConnection>, PoolError> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_url);
    Ok(Pool::builder().build(manager).await?)
}

#[tokio::main]
async fn main() {
    let connection_url = env!("DATABASE_URL").to_string();

    let context = AppContext {
        db: get_connection_pool(connection_url).await.unwrap(),
    };

    let (api_router, openapi) = router::api(context);

    let router = Router::new()
        .merge(api_router)
        .route("/openapi.json", get(Json(openapi)))
        .nest("/docs", RapiDoc::new("/openapi.json").into())
        .fallback((StatusCode::NOT_FOUND, "Not Found"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on port 8080");
    axum::serve(listener, router).await.unwrap();
}
