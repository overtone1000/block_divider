use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use diesel::prelude::*;
use mail::is_valid_email;

use crate::{
    division::{
        block_division::{BlockDivisionBasis, BlockDivisionState},
        bucket::Bucket,
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

    fn get() {}

    fn insert(
        insertion: PersistentDivision,
    ) -> Result<PersistentDivision, Box<dyn std::error::Error>> {
    }

    pub fn get_or_create(
        conn: &mut PgConnection,
        basis: BlockDivisionBasis,
        use_cache_if_available: bool,
    ) -> Result<BlockDivisionState, Box<dyn std::error::Error>> {
        let hash = Self::get_hash(&basis).to_string();

        let results = divisions::table
            .find(&hash)
            .select(PersistentDivision::as_select())
            .load(conn)
            .optional();

        let resulting_persistent_division = match results {
            Ok(Some(results)) => {
                if results.len() > 1 {
                    return Err("More than one division found with the provided hash.".into());
                } else if results.len() < 1 {
                    None
                } else {
                    println!("Results are {:?}", results);
                    Some(results.get(0).expect("Bad check").clone())
                }
            }
            Ok(None) => None,
            Err(e) => return Err(Box::new(e)),
        };

        match resulting_persistent_division {
            Some(result) => Ok(serde_json::from_str(result.serialized.as_str())?),
            None => {
                let retval = BlockDivisionState::create_empty(basis);

                Self::insert(PersistentDivision {
                    hash: hash,
                    serialized: serde_json::to_string(&retval)?,
                });

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
