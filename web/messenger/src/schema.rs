table! {
    users (id) {
        id -> Integer,
        username -> Text,
    }
}

table! {
    messages (id) {
        id -> Integer,
        author_id -> Integer,
        recipient_id -> Integer,
        text -> Text,
        date -> Timestamptz,
    }
}
