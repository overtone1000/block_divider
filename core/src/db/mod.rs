use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod division;
pub mod key_value;
pub mod user;

pub fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&database_url())
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url()))
}

mod tests {
    use super::*;

    #[test]
    fn test_db_connection() {
        dotenvy::dotenv().expect("Couldn't load environment variables for testing.");
        establish_connection();
    }
}
