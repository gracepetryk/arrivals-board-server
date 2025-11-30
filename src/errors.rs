use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel_async::pooled_connection::bb8;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, thiserror::Error)]
#[error("An internal server error occured.")]
pub enum GetRequestError {
    DatabaseError(diesel::result::Error),
    #[error("Requested item not found.")]
    NotFound(diesel::result::Error),
    ConnectionError(#[from] bb8::RunError),
}

impl From<diesel::result::Error> for GetRequestError {
    fn from(value: diesel::result::Error) -> Self {
        match value {
            diesel::result::Error::NotFound => Self::NotFound(value),
            _ => Self::DatabaseError(value),
        }
    }
}

impl IntoResponse for GetRequestError {
    fn into_response(self) -> Response {
        let code = match self {
            GetRequestError::DatabaseError(_) | GetRequestError::ConnectionError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }

            GetRequestError::NotFound(_) => StatusCode::NOT_FOUND,
        };

        eprintln!("{}", self);

        (code, self.to_string()).into_response()
    }
}

pub trait Stringify: Display + Send + Sync + 'static {}
impl<T> Stringify for T where T: Display + Send + Sync + 'static {}

#[serde_as]
#[derive(Debug, utoipa::ToSchema, Serialize)]
pub struct ErrorReason<T: Stringify> {
    #[schema(minimum = 400, maximum = 599)]
    status: u16,
    #[serde_as(as = "DisplayFromStr")]
    reason: T,
}

impl<T: Stringify> ErrorReason<T> {
    pub fn new(status: u16, reason: T) -> Self {
        Self { status, reason }
    }
}

impl<T: Stringify> IntoResponse for ErrorReason<T> {
    fn into_response(self) -> Response {
        let code = StatusCode::try_from(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        (code, Json(self)).into_response()
    }
}
