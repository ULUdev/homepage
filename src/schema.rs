table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        #[sql_name = "priv"]
        priv_ -> Int4,
    }
}
