table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        avatar -> Nullable<Text>,
        memo -> Nullable<Text>,
        last_logined_at -> Timestamp,
    }
}
