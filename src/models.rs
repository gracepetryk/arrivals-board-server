use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{tracking_params, users};

#[derive(
    Queryable,
    Selectable,
    Identifiable,
    Serialize,
    Deserialize,
    utoipa::ToSchema,
    utoipa::ToResponse,
)]
pub struct User {
    id: Uuid,
    email: String,
}

#[derive(
    Associations,
    Queryable,
    Selectable,
    Identifiable,
    Insertable,
    AsChangeset,
    Serialize,
    Deserialize,
)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = tracking_params)]
#[diesel(belongs_to(User))]
pub struct TrackingParams {
    id: Uuid,
    user_id: Uuid,
    min_lat: Option<f64>,
    min_long: Option<f64>,
    max_lat: Option<f64>,
    max_long: Option<f64>,
    origin_iata: Option<String>,
    dest_iata: Option<String>,
}
