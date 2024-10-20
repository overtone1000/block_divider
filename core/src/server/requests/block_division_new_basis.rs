use serde::{Deserialize, Serialize};

use crate::division::{basis::BlockDivisionBasis, state::BlockDivisionState};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct NewBasisRequest {
    id: String,
    basis: BlockDivisionBasis,
}

impl NewBasisRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_basis(&self) -> &BlockDivisionBasis {
        &self.basis
    }
}
