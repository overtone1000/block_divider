use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use tokio::sync::mpsc::{self, Receiver, Sender};

pub mod division;
pub mod user;

pub enum DatabaseTransaction {
    Query,
}
pub struct ConnectionHandler {
    tx: Sender<DatabaseTransaction>,
    rx: Receiver<DatabaseTransaction>,
}

impl ConnectionHandler {
    pub fn new() -> ConnectionHandler {
        let (tx, rx) = mpsc::channel::<DatabaseTransaction>(32);
        ConnectionHandler { tx: tx, rx: rx }
    }

    pub fn getSender(&self) -> Sender<DatabaseTransaction> {
        self.tx.clone()
    }
}

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

mod tests {
    use super::*;

    #[test]
    fn test_db_connection() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        establish_connection();
    }
}
