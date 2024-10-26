use std::hash::{DefaultHasher, Hash, Hasher};

use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::db::key_value::KeyValuePair;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct GetUserViewRequest {
    hash: String,
}

impl GetUserViewRequest {
    pub fn get_hash(&self) -> &str {
        &self.hash
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Hash)]
pub(crate) struct UserView {
    user_id: i32,
    state_id: String,
}

impl UserView {
    pub fn create(user_id: i32, state_id: String) -> UserView {
        UserView {
            user_id: user_id,
            state_id: state_id,
        }
    }
    pub fn get_state_id(&self) -> &str {
        &self.state_id
    }

    pub fn get_user_id(&self) -> i32 {
        self.user_id
    }

    pub fn get_hash(&self) -> String {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish().to_string()
    }

    fn as_json(&self) -> String {
        serde_json::to_string(self).expect("Should serialize.")
    }

    pub fn get(conn: &mut PgConnection, key: &str) -> Result<UserView, Box<dyn std::error::Error>> {
        match KeyValuePair::get(conn, key) {
            Some(str) => match serde_json::from_str::<UserView>(&str) {
                Ok(res) => Ok(res),
                Err(e) => Err(Box::new(e)),
            },
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No such value.",
            ))),
        }
    }

    pub fn set(&self, conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
        match KeyValuePair::set(conn, &self.get_hash(), Some(self.as_json()), false) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
