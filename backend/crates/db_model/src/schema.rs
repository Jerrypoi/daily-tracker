// @generated automatically by Diesel CLI.

diesel::table! {
    daily_track (id) {
        #[max_length = 16]
        id -> Binary,
        start_time -> Datetime,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        #[max_length = 16]
        topic_id -> Nullable<Binary>,
        comment -> Nullable<Text>,
        #[max_length = 16]
        user_id -> Nullable<Binary>,
    }
}

diesel::table! {
    topic (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 255]
        topic_name -> Varchar,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
        #[max_length = 16]
        parent_topic_id -> Nullable<Binary>,
        #[max_length = 7]
        display_color -> Varchar,
        #[max_length = 16]
        user_id -> Nullable<Binary>,
    }
}

diesel::table! {
    users (id) {
        #[max_length = 16]
        id -> Binary,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Datetime,
        updated_at -> Nullable<Datetime>,
    }
}

diesel::joinable!(daily_track -> topic (topic_id));
diesel::joinable!(daily_track -> users (user_id));
diesel::joinable!(topic -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(daily_track, topic, users,);
