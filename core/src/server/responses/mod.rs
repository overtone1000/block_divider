use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::division::state::BlockDivisionState;

pub trait BlockDivisionServerResponse: Serialize {}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct SingleBlockDivisionState {
    pub id: String,
    pub state: BlockDivisionState,
}

impl BlockDivisionServerResponse for SingleBlockDivisionState {}
impl BlockDivisionServerResponse for bool {}
impl BlockDivisionServerResponse for BTreeMap<String, BlockDivisionState> {}
