// @generated automatically by Diesel CLI.

diesel::table! {
    api_keys (id) {
        id -> Bigint,
        user_id -> Bigint,
        #[max_length = 64]
        key_hash -> Varchar,
        #[max_length = 16]
        key_prefix -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        created_at -> Datetime,
        last_used_at -> Nullable<Datetime>,
        revoked_at -> Nullable<Datetime>,
    }
}

diesel::table! {
    daily_track (id) {
        id -> Bigint,
        start_time -> Datetime,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        topic_id -> Nullable<Bigint>,
        comment -> Nullable<Text>,
        user_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    topic (id) {
        id -> Bigint,
        #[max_length = 255]
        topic_name -> Varchar,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        parent_topic_id -> Nullable<Bigint>,
        #[max_length = 7]
        display_color -> Varchar,
        user_id -> Nullable<Bigint>,
    }
}

diesel::table! {
    users (id) {
        id -> Bigint,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        email_verified -> Bool,
        verification_code -> Nullable<Text>,
        verification_code_expires_at -> Nullable<Datetime>,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::joinable!(api_keys -> users (user_id));
diesel::joinable!(daily_track -> topic (topic_id));
diesel::joinable!(daily_track -> users (user_id));
diesel::joinable!(topic -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(api_keys, daily_track, topic, users,);
