use std::fmt::Debug;

use axum::Router;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, bb8::Pool};

mod controllers;
mod models;
mod router;
mod schema;

#[allow(dead_code)]
#[derive(Debug)]
struct AppError(anyhow::Error);

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(value: E) -> Self {
        return Self(value.into());
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let msg_500 = "An internal server error occured.";

        if let Some(status_code) = self.0.downcast_ref::<StatusCode>() {
            let message = match *status_code {
                StatusCode::NOT_FOUND => "Requested resource not found.",
                _ => status_code.canonical_reason().unwrap_or(msg_500),
            };

            return (*status_code, message).into_response();
        } else {
            return (StatusCode::INTERNAL_SERVER_ERROR, msg_500).into_response();
        }
    }
}

#[derive(Clone, Debug)]
struct AppContext {
    db: Pool<AsyncPgConnection>,
}

async fn get_connection_pool(connection_url: String) -> Result<Pool<AsyncPgConnection>, AppError> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(connection_url);
    Ok(Pool::builder().build(manager).await?)
}

#[tokio::main]
async fn main() {
    // this will panic if we can't connect to the db or the environment variable isn't available,
    // nginx will handle the 500 page.
    let connection_url = std::env::var("DATABASE_URL").unwrap();

    let context = AppContext {
        db: get_connection_pool(connection_url).await.unwrap(),
    };

    let api_router = router::api(context);

    let router = api_router.nest("/docs/crate", router::crate_docs());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on port 8080");
    axum::serve(listener, router).await.unwrap();
}
