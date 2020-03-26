table! {
    domains (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        domain_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        email -> Varchar,
        avatar -> Nullable<Text>,
        memo -> Nullable<Text>,
        last_logined_at -> Timestamp,
    }
}

joinable!(roles -> domains (domain_id));

allow_tables_to_appear_in_same_query!(
    domains,
    roles,
    users,
);
