use db::DatabaseTransactable;
use diesel::PgConnection;

use crate::division::block_division::BlockDivisionBasis;
use crate::division::block_division::BlockDivisionState;

use super::division::PersistentDivision;

pub enum DatabaseTransaction {
    GetBlockDivisionState(
        String,
        tokio::sync::oneshot::Sender<Option<BlockDivisionState>>,
    ),
}

impl DatabaseTransactable<PgConnection> for DatabaseTransaction {
    fn handle(self: Self, conn: &mut PgConnection) {
        println!("Handling database transaction.");

        match self {
            DatabaseTransaction::GetBlockDivisionState(basis, responder) => {
                println!("Getting state.");
                let res = PersistentDivision::get_state_from_id(conn, &basis);

                let response = match res {
                    Ok(res) => res,
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                };

                println!("Sending response.");
                responder
                    .send(response)
                    .expect("Could not send to other thread.");
            }
        }
    }
}
