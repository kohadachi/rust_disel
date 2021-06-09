table! {
    articles (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
    }
}

table! {
    users (id) {
        id -> Unsigned<Bigint>,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    users,
);
