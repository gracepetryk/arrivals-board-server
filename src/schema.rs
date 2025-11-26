// @generated automatically by Diesel CLI.

diesel::table! {
    tracking_params (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 3]
        origin_iata -> Nullable<Bpchar>,
        #[max_length = 3]
        dest_iata -> Nullable<Bpchar>,
        updated_at -> Nullable<Timestamp>,
        min_lat -> Nullable<Float8>,
        min_long -> Nullable<Float8>,
        max_lat -> Nullable<Float8>,
        max_long -> Nullable<Float8>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
    }
}

diesel::joinable!(tracking_params -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(tracking_params, users,);
