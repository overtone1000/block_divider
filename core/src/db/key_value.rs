use diesel::prelude::*;
use mail::is_valid_email;

use crate::schema::{key_val_store, users};

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::key_val_store)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct KeyValuePair {
    key: String,
    value: String,
}

impl KeyValuePair {
    pub fn set(
        conn: &mut PgConnection,
        key: &str,
        value: Option<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match value {
            Some(value) => match diesel::insert_into(key_val_store::table)
                .values(KeyValuePair {
                    key: key.to_string(),
                    value: value.clone(),
                })
                .execute(conn)
            {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            },
            None => match KeyValuePair::delete(conn, key) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            },
        }
    }

    pub fn get(conn: &mut PgConnection, key: &str) -> Option<String> {
        let result = key_val_store::table
            .find(key)
            .select(KeyValuePair::as_select())
            .first(conn)
            .optional()
            .expect("Should always return.");

        match result {
            Some(result) => Some(result.value),
            None => None,
        }
    }

    fn delete(conn: &mut PgConnection, key: &str) -> Result<usize, diesel::result::Error> {
        diesel::delete(key_val_store::table.find(key)).execute(conn)
    }
}

mod tests {
    use crate::db::establish_connection;

    use super::*;

    #[test]
    fn test_insert_and_delete() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        let conn = &mut establish_connection();

        let skvp = KeyValuePair {
            key: "test_some".to_string(),
            value: "test some value".to_string(),
        };

        let test_none_key = "test_none";

        KeyValuePair::set(conn, test_none_key, None).expect("Should be able to set.");
        assert!(None == KeyValuePair::get(conn, test_none_key));

        KeyValuePair::set(conn, &skvp.key, Some(skvp.value.to_string()))
            .expect("Should be able to set.");
        assert!(Some(skvp.value) == KeyValuePair::get(conn, &skvp.key));
        KeyValuePair::set(conn, &skvp.key, None).expect("Should be able to set.");
    }
}
