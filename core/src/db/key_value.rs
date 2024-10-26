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
        new_value: Option<String>,
        allow_overwrite: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match new_value {
            Some(new_value) => {
                let insert_operation =
                    diesel::insert_into(key_val_store::table).values(KeyValuePair {
                        key: key.to_string(),
                        value: new_value.clone(),
                    });

                let result = match allow_overwrite {
                    true => insert_operation
                        .on_conflict(key_val_store::key)
                        .do_update()
                        .set(key_val_store::value.eq(new_value))
                        .execute(conn),
                    false => insert_operation.execute(conn),
                };

                match result {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(e)),
                }
            }
            None => match allow_overwrite {
                true => match KeyValuePair::delete(conn, key) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Box::new(e)),
                },
                false => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    "Deletion requested without write access.",
                ))),
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

        KeyValuePair::set(conn, &skvp.key, None, true).expect("Should be able to set.");
        assert!(None == KeyValuePair::get(conn, test_none_key));
        KeyValuePair::set(conn, test_none_key, None, true).expect("Should be able to set.");
        assert!(None == KeyValuePair::get(conn, test_none_key));

        KeyValuePair::set(conn, &skvp.key, Some(skvp.value.to_string()), false)
            .expect("Should be able to set.");
        assert!(Some(skvp.value) == KeyValuePair::get(conn, &skvp.key));

        let should_fail =
            KeyValuePair::set(conn, &skvp.key, Some("Shouldn't work!".to_string()), false);
        assert!(should_fail.is_err());

        let should_fail = KeyValuePair::set(conn, &skvp.key, None, false);
        assert!(should_fail.is_err());

        KeyValuePair::set(conn, &skvp.key, Some("Should work!".to_string()), true)
            .expect("Should be able to set.");

        KeyValuePair::set(conn, &skvp.key, None, true).expect("Should be able to set.");
    }
}
