table! {
    content (name) {
        name -> Varchar,
        content_inner -> Bytea,
        mime_type -> Varchar,
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

allow_tables_to_appear_in_same_query!(content, users,);
