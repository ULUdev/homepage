table! {
    tokens (uid) {
        uid -> Int4,
        token -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        pwd -> Varchar,
        privs -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    tokens,
    users,
);
