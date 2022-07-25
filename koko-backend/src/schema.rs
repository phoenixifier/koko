table! {
    anime (title) {
        title -> Varchar,
        genre -> Varchar,
        year -> Int4,
        description -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    users (email) {
        email -> Varchar,
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(anime, users);
