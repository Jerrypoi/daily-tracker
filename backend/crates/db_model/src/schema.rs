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
    }
}

diesel::joinable!(daily_track -> topic (topic_id));

diesel::allow_tables_to_appear_in_same_query!(daily_track, topic,);
