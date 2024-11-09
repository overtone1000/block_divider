use serde::{Deserialize, Serialize};

use crate::division::state::BlockDivisionState;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct SetOpenRoundRequest {
    id: String,
    round: Option<usize>,
}

impl SetOpenRoundRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_round(&self) -> &Option<usize> {
        &self.round
    }
}
