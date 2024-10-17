use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use diesel::prelude::*;
use mail::is_valid_email;

use crate::{
    division::block_division::{BlockDivisionBasis, BlockDivisionState},
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
    fn get_hash(basis: &BlockDivisionBasis) -> String {
        let mut hasher = std::hash::DefaultHasher::new();
        for (bucket_name, bucket_definition) in &basis.bucket_definitions {
            bucket_name.hash(&mut hasher);
            bucket_definition.hash(&mut hasher);
        }
        basis.participant_round_picks.hash(&mut hasher);
        basis.selection_rounds.hash(&mut hasher);
        hasher.finish().to_string()
    }

    pub fn get_from_basis(
        conn: &mut PgConnection,
        basis: &BlockDivisionBasis,
    ) -> Option<PersistentDivision> {
        let hash = Self::get_hash(&basis);
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
            hash: Self::get_hash(&state.basis),
            serialized: serde_json::to_string(&state)?,
        };

        diesel::update(divisions::table.find(update.hash))
            .set(divisions::serialized.eq(update.serialized))
            .execute(conn)?;

        Ok(())
    }

    pub fn get_or_create(
        conn: &mut PgConnection,
        basis: &BlockDivisionBasis,
    ) -> Result<BlockDivisionState, Box<dyn std::error::Error>> {
        let hash = Self::get_hash(&basis);

        let resulting_persistent_division = Self::get_from_basis(conn, basis);

        match resulting_persistent_division {
            Some(result) => {
                let str = result.serialized.as_str();
                Ok(serde_json::from_str(str)?)
            }
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
        let hash = Self::get_hash(basis);
        Self::delete_division(conn, &hash)
    }
}
