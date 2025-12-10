use std::fmt::Debug;

use diesel_async::pooled_connection::bb8;
use serde::Serialize;
use utoipa::{PartialSchema, ToSchema};

#[derive(Debug, thiserror::Error)]
#[error("An internal server error occured.")]
pub enum InternalServerErrorReason {
    DatabaseError(#[from] diesel::result::Error),
    ConnectionError(#[from] bb8::RunError),
}

impl PartialSchema for InternalServerErrorReason {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        "internal server error.".into()
    }
}

impl ToSchema for InternalServerErrorReason {}

#[derive(Debug, ToSchema, Serialize)]
pub struct ReasonResponse<T: ToSchema + Serialize> {
    reason: T,
}

impl<T: ToSchema + Serialize> From<T> for ReasonResponse<T> {
    fn from(reason: T) -> Self {
        ReasonResponse { reason }
    }
}

macro_rules! error_enum {
    ($enum_name:ident {
        $(
            #[response(status = $status:path, description = $desc:literal $($_resp_attr:meta),* $(,)?)]
            $variant:ident $(($field:tt))?
        ),* $(,)?
    }) => {
        #[derive(Debug, utoipa::IntoResponses, thiserror::Error)]
        enum $enum_name {
            #[response(status = http::status::StatusCode::INTERNAL_SERVER_ERROR, description = "Internal server error.")]
            #[error("Internal server error.")]
            InternalServerError(#[to_schema] $crate::errors::InternalServerErrorReason),
            $(
                #[response(status = $status, description = $desc, $($_resp_attr)*)]
                #[error($desc)]
                #[allow(dead_code)]
                $variant $((#[to_schema] $crate::errors::ReasonResponse<$field>))?
            ),*
        }

        impl<T: Into<$crate::errors::InternalServerErrorReason>> From<T> for $enum_name {
            fn from(value: T) -> Self {
                <$enum_name>::InternalServerError(value.into())
            }
        }

        impl axum::response::IntoResponse for $enum_name {
            fn into_response(self) -> axum::response::Response {
                macro_rules! match_arm {

                    ($variant_status:path, $variant_desc:literal) => {
                        ($variant_status, $variant_desc).into_response()
                    };

                    ($variant_status:path, $variant_desc:literal, $variant_field:tt) => {
                        ($variant_status, axum::Json($variant_field)).into_response()
                    }

                }
                match self {
                    $enum_name::InternalServerError(_) => {
                        (http::status::StatusCode::INTERNAL_SERVER_ERROR, "internal server error.").into_response()
                    },
                    $(
                        #[allow(non_snake_case)]
                        $enum_name::$variant$(($field))? => match_arm!($status, $desc $(,$field)?)
                    ),*
                }
            }
        }
    };
}

pub(crate) use error_enum;
