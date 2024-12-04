diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    trades (id) {
        id -> Int4,
        creator_id -> Int4,
        amount -> Int8,
        price -> Float8,
        status -> Varchar,
        created_at -> Timestamp,
    }
}
