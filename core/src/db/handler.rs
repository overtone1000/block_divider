use db::DatabaseTransactable;
use diesel::PgConnection;

use crate::{
    division::state::BlockDivisionState,
    server::requests::block_division_set_state::SetStateRequest,
};

use super::division::PersistentDivision;

pub enum DatabaseTransaction {
    GetBlockDivisionState(
        String,
        tokio::sync::oneshot::Sender<Option<BlockDivisionState>>,
    ),
    GetBlockDivisionList(tokio::sync::oneshot::Sender<Option<Vec<(String, BlockDivisionState)>>>),
    SetBlockDivisionState(SetStateRequest, tokio::sync::oneshot::Sender<Option<bool>>),
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
            DatabaseTransaction::GetBlockDivisionList(responder) => {
                println!("Getting list.");
                let res = PersistentDivision::get_all(conn)
                    .expect("Couldn't get all from persistent division table.");

                println!("Sending response.");
                responder
                    .send(Some(res))
                    .expect("Could not send to other thread.");
            }
            DatabaseTransaction::SetBlockDivisionState(request, responder) => {
                println!("Setting persistent division.");
                let res = match PersistentDivision::update(conn, request.get_state()) {
                    Ok(_) => true,
                    Err(_) => false,
                };
                println!("Sending response.");
                responder
                    .send(Some(res))
                    .expect("Could not send to other thread.");
            }
        }
    }
}
