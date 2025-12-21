use std::fmt::Debug;

use diesel_async::pooled_connection::bb8;
use serde::Serialize;
use utoipa::{PartialSchema, ToResponse, ToSchema};

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

#[derive(Debug, ToSchema, ToResponse, Serialize)]
pub struct ReasonResponse<T: ToSchema + Serialize> {
    #[schema(inline)]
    reason: T,
}

impl<T: ToSchema + Serialize> From<T> for ReasonResponse<T> {
    fn from(reason: T) -> Self {
        ReasonResponse { reason }
    }
}

macro_rules! error_enum {
    ($v:vis $enum_name:ident {
        $(
            #[response(status = $status:path, description = $desc:literal $($_resp_attr:meta),* $(,)?)]
            $variant:ident $(($field:ty))?
        ),* $(,)?
    }) => {
        #[derive(Debug, thiserror::Error)]
        $v enum $enum_name {
            #[error("Internal server error.")]
            InternalServerError($crate::errors::InternalServerErrorReason),
            $(
                #[error($desc)]
                #[allow(dead_code)]
                $variant $(($crate::errors::ReasonResponse<$field>))?
            ),*
        }

        impl<T: Into<$crate::errors::InternalServerErrorReason>> From<T> for $enum_name {
            fn from(value: T) -> Self {
                <$enum_name>::InternalServerError(value.into())
            }
        }

        impl utoipa::IntoResponses for $enum_name {
            #[allow(unused_imports)]
            fn responses() -> std::collections::BTreeMap<String, utoipa::openapi::RefOr<utoipa::openapi::response::Response>> {
                use utoipa::openapi::{ResponsesBuilder, ResponseBuilder, ContentBuilder};
                use utoipa::PartialSchema;
                use $crate::errors::ReasonResponse;

                ResponsesBuilder::new()
                    $(
                        .response(
                        $status.to_string(),
                        ResponseBuilder::new()
                        .description($desc)
                        $( .content("application/json", ContentBuilder::new()
                            .schema(Some(ReasonResponse::<$field>::schema()))
                            .build()) )?
                        )

                    )*
                    .build().into()
            }
        }

        impl axum::response::IntoResponse for $enum_name {
            fn into_response(self) -> axum::response::Response {
                macro_rules! match_pattern {
                    ($enum_name::$variant_name:ident) => { $enum_name::$variant_name };
                    ($enum_name::$variant_name:ident($variant_field:ty, $capture:ident)) => {
                        $enum_name::$variant_name($capture)
                    }
                }

                macro_rules! match_arm {

                    ($variant_status:path, $variant_desc:literal) => {
                        ($variant_status, $variant_desc).into_response()
                    };

                    ($variant_status:path, $variant_desc:literal, $variant_field:ty, $capture:ident) => {
                        ($variant_status, axum::Json($capture)).into_response()
                    }
                }

                match self {
                    $enum_name::InternalServerError(_) => {
                        (
                            http::status::StatusCode::INTERNAL_SERVER_ERROR,
                            "internal server error."
                        ).into_response()
                    },
                    $(
                        match_pattern!( $enum_name::$variant $( ($field, t) )? )=> {
                            match_arm!($status, $desc $(,$field, t)?)
                        }
                    ),*
                }
            }
        }
    };
}

pub(crate) use error_enum;
