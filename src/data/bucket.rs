use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::participant::Participant;

pub type Designations = HashMap<u64, Vec<Participant>>; //Map containing predesignations in index 0 and each selection round index after that
pub type Ranks = HashMap<u64, HashMap<Participant, u64>>;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct BucketDef {
    pub(crate) available_slots: u64, //How many participants can fit in this bucket in total
    pub(crate) available_ancillaries: Vec<String>, //What ancillaries are available to an individual participant in this bucket
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct BucketState {
    pub(crate) designations: Designations, //Who is in this bucket in each round (0 is predesignation, 1 is round 1, 2 is round 2, etc.)
    pub(crate) ranks: Ranks, //Map containing the ranks for each participant for each potential round
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Bucket {
    pub(crate) definition: BucketDef,
    pub(crate) state: BucketState,
}
