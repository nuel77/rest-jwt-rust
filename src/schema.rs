// @generated automatically by Diesel CLI.

diesel::table! {
    transactions (id) {
        id -> Int4,
        from_user -> Int4,
        to_user -> Int4,
        amount -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        balance -> Int4,
        removed -> Bool,
        password -> Varchar,
        session_token -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    transactions,
    users,
);
