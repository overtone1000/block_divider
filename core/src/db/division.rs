use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use diesel::prelude::*;
use mail::is_valid_email;

use crate::{
    division::{
        block_division::{BlockDivisionBasis, BlockDivisionState},
        bucket::BucketStates,
    },
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
    fn get_hash(basis: &BlockDivisionBasis) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        for (bucket_name, bucket_definition) in &basis.bucket_definitions {
            bucket_name.hash(&mut hasher);
            bucket_definition.hash(&mut hasher);
        }
        basis.participant_round_picks.hash(&mut hasher);
        basis.selection_rounds.hash(&mut hasher);
        hasher.finish()
    }

    fn get(conn: &mut PgConnection, basis: &BlockDivisionBasis) -> Option<PersistentDivision> {
        let hash = Self::get_hash(&basis).to_string();
        divisions::table
            .find(hash)
            .select(PersistentDivision::as_select())
            .first(conn)
            .optional()
            .expect("Should return option.")
    }

    fn insert(
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
            hash: Self::get_hash(&state.basis).to_string(),
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
        use_persistent_if_exists: bool,
    ) -> Result<BlockDivisionState, Box<dyn std::error::Error>> {
        let hash = Self::get_hash(&basis).to_string();

        let resulting_persistent_division = match use_persistent_if_exists {
            true => Self::get(conn, basis),
            false => None,
        };

        match resulting_persistent_division {
            Some(result) => Ok(serde_json::from_str(result.serialized.as_str())?),
            None => {
                let retval = BlockDivisionState::create_empty(basis);

                Self::insert(
                    conn,
                    PersistentDivision {
                        hash: hash,
                        serialized: serde_json::to_string(&retval)?,
                    },
                );

                Ok(retval)
            }
        }
    }

    fn delete_division(
        conn: &mut PgConnection,
        email: &str,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(divisions::table.find(email)).execute(conn)
    }
}
