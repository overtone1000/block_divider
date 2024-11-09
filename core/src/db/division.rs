use std::collections::BTreeMap;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::{
    division::{basis::BlockDivisionBasis, state::BlockDivisionState},
    schema::divisions,
};

#[derive(Queryable, Selectable, Insertable, Debug, PartialEq, Clone)]
#[diesel(table_name = crate::schema::divisions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PersistentDivision {
    id: String,
    serialized: String,
}

impl PersistentDivision {
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }

    //pub fn get_state(&self) -> BlockDivisionState {
    //    serde_json::from_str(&self.serialized).expect("Couldn't deserialize persistent division.")
    //}

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

    //Returns a vector of tuples containing (Hash,BlockDivisionState)
    pub fn get_all(
        conn: &mut PgConnection,
    ) -> Result<BTreeMap<String, BlockDivisionState>, Box<dyn std::error::Error>> {
        let result = divisions::table
            .select(PersistentDivision::as_select())
            .load(conn);

        match result {
            Ok(result) => Ok(result
                .iter()
                .map(|pd| {
                    (
                        pd.id.clone(),
                        pd.as_state()
                            .expect("Couldn't convert persistent division to state."),
                    )
                })
                .collect::<BTreeMap<String, BlockDivisionState>>()),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn update(
        conn: &mut PgConnection,
        id: String,
        state: &BlockDivisionState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let update = PersistentDivision {
            id: id.to_string(),
            serialized: serde_json::to_string(&state)?,
        };

        diesel::update(divisions::table.find(id))
            .set(divisions::serialized.eq(update.serialized))
            .execute(conn)?;

        Ok(())
    }

    pub fn as_state(&self) -> Result<BlockDivisionState, Box<dyn std::error::Error>> {
        let str = self.serialized.as_str();
        Ok(serde_json::from_str(str)?)
    }

    pub fn new(
        conn: &mut PgConnection,
        id: String,
        basis: &BlockDivisionBasis,
    ) -> Result<PersistentDivision, Box<dyn std::error::Error>> {
        let new_state = BlockDivisionState::new(basis);
        let insertion = PersistentDivision {
            id: id,
            serialized: serde_json::to_string(&new_state)?,
        };

        diesel::insert_into(divisions::table)
            .values(&insertion)
            .execute(conn)
            .expect("Error creating new persistent division");

        Ok(insertion)
    }

    pub fn delete_division(
        conn: &mut PgConnection,
        id: String,
    ) -> Result<usize, diesel::result::Error> {
        diesel::delete(divisions::table.find(id)).execute(conn)
    }
}
