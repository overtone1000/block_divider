use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{participant::Participant, selections::Selections, week::Week};

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    selections: HashMap<Participant, Selections>,
}

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionState {
    weeks: Vec<Week>,                       //Weeks being selected
    ranks: HashMap<Week, Vec<Participant>>, //participant ranks for each week
}
