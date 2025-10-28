use axum::{http::StatusCode, response::IntoResponse};
use diesel_async::{AsyncPgConnection, pooled_connection::bb8::Pool};

mod controllers;
mod models;
mod router;
mod schema;

type ConnectionPool = Pool<AsyncPgConnection>;

#[allow(dead_code)]
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal server error occured.",
        )
            .into_response()
    }
}

#[derive(Clone)]
struct AppContext {
    db: ConnectionPool,
}

fn get_connection_pool() -> ConnectionPool {
    todo!()
}

#[tokio::main]
async fn main() {
    let db = AppContext {
        db: get_connection_pool(),
    };
    let app = router::route(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
