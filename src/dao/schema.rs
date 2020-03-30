table! {
    actions (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        domain_id -> Int4,
    }
}

table! {
    domains (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
    }
}

table! {
    role_has_actions (action_id, role_id) {
        role_id -> Int4,
        action_id -> Int4,
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
    user_has_roles (user_id, role_id) {
        user_id -> Uuid,
        role_id -> Int4,
        expire -> Timestamp,
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

joinable!(actions -> domains (domain_id));
joinable!(role_has_actions -> actions (action_id));
joinable!(role_has_actions -> roles (role_id));
joinable!(roles -> domains (domain_id));
joinable!(user_has_roles -> roles (role_id));
joinable!(user_has_roles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    actions,
    domains,
    role_has_actions,
    roles,
    user_has_roles,
    users,
);
