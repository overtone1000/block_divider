use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod division;
pub mod handler;
pub mod user;

pub fn database_url() -> String {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
}

pub fn establish_connection() -> PgConnection {
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url()))
}

mod tests {
    use super::*;

    #[test]
    fn test_db_connection() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        establish_connection();
    }
}
