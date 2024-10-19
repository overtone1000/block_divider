use serde::{Deserialize, Serialize};

use crate::division::state::BlockDivisionState;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SetStateRequest {
    state: BlockDivisionState,
}

impl SetStateRequest {
    pub fn get_state(&self) -> &BlockDivisionState {
        &self.state
    }
}
