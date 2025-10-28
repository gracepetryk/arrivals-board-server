use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::{tracking_params, users};

#[derive(Queryable, Selectable, Identifiable, Serialize)]
pub struct User {
    id: Uuid,
    email: String,
    pw_hash: Option<String>,
}

#[derive(Queryable, Selectable, Associations, Identifiable, Serialize)]
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

pub fn foo(id: Uuid, conn: &mut PgConnection) {
    let user = users::table
        .select(User::as_select())
        .first(conn)
        .expect("foo");

    let x = TrackingParams::belonging_to(&user)
        .select(TrackingParams::as_select())
        .find(id)
        .first(conn)
        .expect("foo");
}
