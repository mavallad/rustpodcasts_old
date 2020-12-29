table! {
    episodes (id) {
        id -> Text,
        id_serie -> Nullable<Text>,
        podcast_title -> Text,
        title -> Text,
        description -> Nullable<Text>,
        interviewed -> Nullable<Text>,
        publication_date -> Nullable<Date>,
        image_url -> Nullable<Text>,
        link -> Text,
        duration_seconds -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    series (id) {
        id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        image_url -> Nullable<Text>,
        link -> Text,
        status -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(episodes -> series (id_serie));

allow_tables_to_appear_in_same_query!(
    episodes,
    series,
);
