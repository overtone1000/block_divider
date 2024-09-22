// @generated automatically by Diesel CLI.

diesel::table! {
    users (email) {
        email -> Varchar,
        hashed_password -> Nullable<Varchar>,
        display_name -> Varchar,
    }
}
