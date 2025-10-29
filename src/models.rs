use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{tracking_params, users};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize)]
pub struct User {
    id: Uuid,
    email: String,
}

#[derive(Insertable, AsChangeset, Serialize)]
#[diesel(table_name = users)]
pub struct AuthUser {
    id: Uuid,
    email: String,
    pw_hash: String,
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
    min_lat: Option<f32>,
    min_long: Option<f32>,
    max_lat: Option<f32>,
    max_long: Option<f32>,
    origin_iata: Option<String>,
    dest_iata: Option<String>,
}
