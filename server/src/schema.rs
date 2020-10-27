table! {
    messages (id) {
        id -> Int4,
        from -> Nullable<Varchar>,
        to -> Varchar,
        body -> Varchar,
        timestamp -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(messages, users,);
