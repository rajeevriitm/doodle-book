table! {
    drawings (id) {
        id -> Int4,
        points -> Text,
        width -> Int4,
        created_at -> Timestamp,
        user_id -> Int4,
    }
}

table! {
    relationships (id) {
        id -> Int4,
        follower_id -> Int4,
        following_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        profile_pic -> Nullable<Text>,
        profile_pic_width -> Nullable<Int4>,
        followers_count -> Int4,
        following_count -> Int4,
    }
}

joinable!(drawings -> users (user_id));

allow_tables_to_appear_in_same_query!(
    drawings,
    relationships,
    users,
);
