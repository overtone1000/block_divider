use db::DatabaseTransactable;
use diesel::PgConnection;

use crate::division::block_division::BlockDivisionBasis;
use crate::division::block_division::BlockDivisionState;

use super::division::PersistentDivision;

pub enum DatabaseTransaction {
    PersistentDivision_Get(String),
    PersistentDivision_Insert(PersistentDivision),
    PersistentDivision_Update(BlockDivisionState),
    PersistentDivision_GetOrCreate(BlockDivisionBasis),
    PersistentDivision_Delete(String),
}

impl DatabaseTransactable<PgConnection> for DatabaseTransaction {
    fn handle(&self, conn: &mut PgConnection) {
        match self {
            DatabaseTransaction::PersistentDivision_Get(basis) => {
                PersistentDivision::get_from_id(conn, basis);
            }
            DatabaseTransaction::PersistentDivision_Insert(division) => {
                PersistentDivision::insert(conn, division.clone());
            }
            DatabaseTransaction::PersistentDivision_Update(state) => {
                PersistentDivision::update(conn, state);
            }
            DatabaseTransaction::PersistentDivision_GetOrCreate(basis) => {
                PersistentDivision::get_or_create(conn, basis);
            }
            DatabaseTransaction::PersistentDivision_Delete(hash) => {
                PersistentDivision::delete_division(conn, hash);
            }
        }
    }
}
