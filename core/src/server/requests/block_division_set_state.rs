use serde::{Deserialize, Serialize};

use crate::division::state::BlockDivisionState;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SetStateRequest {
    id: String,
    state: BlockDivisionState,
}

impl SetStateRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_state(&self) -> &BlockDivisionState {
        &self.state
    }
}
