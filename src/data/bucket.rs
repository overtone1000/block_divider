use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::participant::Participant;

pub type Designations = Vec<Participant>; //Map containing selected participants. Keys are rounds of the selection.
pub type Ranks = BTreeMap<Participant, u64>;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct BucketDef {
    pub(crate) available_slots: u64, //How many participants can fit in this bucket in total
    pub(crate) available_ancillaries: BTreeSet<String>, //What ancillaries are available to an individual participant in this bucket
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct BucketState {
    pub(crate) designations: Designations, //Who is in this bucket in each round (0 is predesignation, 1 is round 1, 2 is round 2, etc.)
    pub(crate) ranks: Ranks, //Map containing the ranks for each participant for each potential round
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Bucket {
    pub(crate) definition: BucketDef,
    pub(crate) state: BTreeMap<u64, BucketState>, //the state of each round in this bucket
}
