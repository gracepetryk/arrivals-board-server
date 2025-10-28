// @generated automatically by Diesel CLI.

diesel::table! {
    tracking_params (id) {
        id -> Uuid,
        user_id -> Uuid,
        min_lat -> Nullable<Float>,
        min_long -> Nullable<Float>,
        max_lat -> Nullable<Float>,
        max_long -> Nullable<Float>,
        #[max_length = 3]
        origin_iata -> Nullable<Bpchar>,
        #[max_length = 3]
        dest_iata -> Nullable<Bpchar>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        pw_hash -> Nullable<Varchar>,
    }
}

diesel::joinable!(tracking_params -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(tracking_params, users,);
