table! {
    drawings (id) {
        id -> Int4,
        points -> Text,
        width -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        profile_pic -> Nullable<Text>,
    }
}

joinable!(drawings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    drawings,
    users,
);
