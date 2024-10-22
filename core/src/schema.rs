// @generated automatically by Diesel CLI.

diesel::table! {
    divisions (id) {
        id -> Text,
        serialized -> Text,
    }
}

diesel::table! {
    key_val_store (key) {
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    users (email) {
        email -> Text,
        hashed_password -> Nullable<Varchar>,
        display_name -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    divisions,
    key_val_store,
    users,
);
