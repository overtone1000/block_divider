use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use diesel::prelude::*;
use mail::is_valid_email;

use crate::{
    division::{basis::BlockDivisionBasis, state::BlockDivisionState},
    schema::divisions,
};

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PersistentDivision {
    hash: String,
    serialized: String,
}

impl PersistentDivision {
    pub fn get_from_basis(
        conn: &mut PgConnection,
        basis: &BlockDivisionBasis,
    ) -> Option<PersistentDivision> {
        let hash = basis.get_hash();
        Self::get_from_id(conn, &hash)
    }

    pub fn get_from_id(conn: &mut PgConnection, id: &str) -> Option<PersistentDivision> {
        let retval = divisions::table
            .find(id)
            .select(PersistentDivision::as_select())
            .first(conn)
            .optional()
            .expect("Should return option.");

        retval
    }

    pub fn get_state_from_id(
        conn: &mut PgConnection,
        id: &str,
    ) -> Result<Option<BlockDivisionState>, Box<dyn std::error::Error>> {
        let pd = PersistentDivision::get_from_id(conn, id);
        match pd {
            Some(pd) => Ok(Some(pd.as_state()?)),
            None => Ok(None),
        }
    }

    pub fn get_all(
        conn: &mut PgConnection,
    ) -> Result<Vec<BlockDivisionState>, Box<dyn std::error::Error>> {
        let result = divisions::table
            .select(PersistentDivision::as_select())
            .load(conn);

        match result {
            Ok(result) => Ok(result
                .iter()
                .map(|pd| {
                    pd.as_state()
                        .expect("Couldn't convert persistent division to state.")
                })
                .collect()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn insert(
        conn: &mut PgConnection,
        insertion: PersistentDivision,
    ) -> Result<(), Box<dyn std::error::Error>> {
        diesel::insert_into(divisions::table)
            .values(&insertion)
            .execute(conn)?;

        Ok(())
    }

    pub fn update(
        conn: &mut PgConnection,
        state: &BlockDivisionState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let update = PersistentDivision {
            hash: state.basis.get_hash(),
            serialized: serde_json::to_string(&state)?,
        };

        diesel::update(divisions::table.find(update.hash))
            .set(divisions::serialized.eq(update.serialized))
            .execute(conn)?;

        Ok(())
    }

    fn as_state(&self) -> Result<BlockDivisionState, Box<dyn std::error::Error>> {
        let str = self.serialized.as_str();
        Ok(serde_json::from_str(str)?)
    }

    pub fn get_or_create(
        conn: &mut PgConnection,
        basis: &BlockDivisionBasis,
    ) -> Result<BlockDivisionState, Box<dyn std::error::Error>> {
        let hash = basis.get_hash();

        let resulting_persistent_division = Self::get_from_basis(conn, basis);

        match resulting_persistent_division {
            Some(result) => result.as_state(),
            None => {
                let retval = BlockDivisionState::new(basis);

                Self::insert(
                    conn,
                    PersistentDivision {
                        hash: hash,
                        serialized: serde_json::to_string(&retval)?,
                    },
                )?;

                Ok(retval)
            }
        }
    }

    pub fn delete_division(
        conn: &mut PgConnection,
        hash: &str,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(divisions::table.find(hash)).execute(conn)
    }

    pub fn delete_division_from_basis(
        conn: &mut PgConnection,
        basis: &BlockDivisionBasis,
    ) -> Result<usize, diesel::result::Error> {
        let hash = basis.get_hash();
        Self::delete_division(conn, &hash)
    }
}
