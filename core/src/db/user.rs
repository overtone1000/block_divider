use diesel::prelude::*;
use mail::is_valid_email;

use crate::schema::users;

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    email: String,
    hashed_password: Option<String>,
    display_name: String,
}

impl User {
    pub fn new_user(
        conn: &mut PgConnection,
        email: &str,
        hashed_password: Option<String>,
        display_name: &str,
    ) -> Result<User, Box<dyn std::error::Error>> {
        let stored_address = email.to_lowercase();
        if !is_valid_email(&stored_address) {
            return Err("Invalid e-mail address.".into());
        }

        let new_user = User {
            email: stored_address,
            hashed_password: hashed_password,
            display_name: display_name.to_string(),
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(conn)?;

        Ok(new_user)
    }

    pub fn get_user(conn: &mut PgConnection, email: &str) -> Option<User> {
        users::table
            .find(email)
            .select(User::as_select())
            .first(conn)
            .optional()
            .expect("Should return option.")
    }

    pub fn delete_user(
        conn: &mut PgConnection,
        email: &str,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(users::table.find(email)).execute(conn)
    }
}

mod tests {
    use crate::db::establish_connection;

    use super::*;

    #[test]
    fn test_insert_and_delete() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        let conn = &mut establish_connection();

        let test_user = User::new_user(
            conn,
            "nobody@nobody.com",
            Some("12345".to_string()),
            "Nobody Here",
        )
        .expect("Couldn't create test user.");

        let get_test_user = User::get_user(conn, &test_user.email).expect("Should contain user.");

        assert_eq!(test_user, get_test_user);

        let delete_count =
            User::delete_user(conn, &test_user.email).expect("Should be able to delete user.");

        assert!(delete_count == 1);

        let get_test_user = User::get_user(conn, &test_user.email);

        assert!(get_test_user.is_none());
    }
}
