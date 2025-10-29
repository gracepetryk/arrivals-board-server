use std::fmt::Debug;

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
        let resp = match self.0.downcast() {
            Ok(diesel::NotFound) => (StatusCode::NOT_FOUND, "Requested resource not found."),
            Err(_) | Ok(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error occured.",
            ),
        };

        resp.into_response()
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
    // this will crash if we can't connect to the db or the environment variable isn't available,
    // but it'll be ok to just let nginx handle the error page for now.
    let connection_url = std::env::var("DATABASE_URL").unwrap();

    let context = AppContext {
        db: get_connection_pool(connection_url).await.unwrap(),
    };

    let app = router::route(context);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
