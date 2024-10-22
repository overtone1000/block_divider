use block_division_delete::DeleteStateRequest;
use block_division_list::GetListRequest;
use block_division_new_basis::NewBasisRequest;
use block_division_set_state::SetStateRequest;
use block_division_state::GetStateRequest;
use serde::{Deserialize, Serialize};

pub(crate) mod block_division_delete;
pub(crate) mod block_division_list;
pub(crate) mod block_division_new_basis;
pub(crate) mod block_division_set_state;
pub(crate) mod block_division_state;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) enum BlockDivisionPost {
    GetState(GetStateRequest),
    GetDivisions(GetListRequest),
    SetState(SetStateRequest),
    NewBasis(NewBasisRequest),
    DeleteState(DeleteStateRequest),
}
