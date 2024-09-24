use std::collections::{BTreeMap, BTreeSet, HashSet};

use serde::{Deserialize, Serialize};

use super::{participant::Participant, selections::Selection};

pub type Designations = BTreeSet<Participant>; //Map containing selected participants. Keys are rounds of the selection.
pub type Ranks = BTreeMap<Participant, u64>;

pub type BucketName = String;
pub type AncillaryName = String;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct BucketDef {
    pub(crate) available_slots: u64, //How many participants can fit in this bucket in total
    pub(crate) available_ancillaries: BTreeSet<AncillaryName>, //What ancillaries are available to an individual participant in this bucket
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct BucketState {
    pub(crate) designations: Designations, //Who is in this bucket in each round (0 is predesignation, 1 is round 1, 2 is round 2, etc.)
    pub(crate) ancillary_designations: BTreeMap<BucketName, Participant>,
    pub(crate) ranks: Ranks, //Map containing the ranks for each participant for each potential round
}

impl BucketState {
    pub(crate) fn get_winners(
        &self,
        candidates: &BTreeSet<Participant>,
        winner_count: u64,
    ) -> BTreeSet<Participant> {
        let mut map: BTreeMap<&u64, Participant> = BTreeMap::new(); //automatically sorted by key
        for candidate in candidates {
            let rank = self.ranks.get(candidate).expect("Should exist.");
            map.insert(rank, candidate.to_string());
        }

        while map.len() as u64 > winner_count {
            map.pop_last();
        }

        let mut retval: BTreeSet<Participant> = BTreeSet::new();
        for participant in map.values() {
            retval.insert(participant.to_string());
        }
        retval
    }

    pub(crate) fn is_winner(&self, contender: &Participant, opponent: &Participant) -> bool {
        if self.ranks.get(contender).expect("Should exist")
            > self.ranks.get(opponent).expect("Should exist")
        {
            false
        } else {
            true
        }
    }
}

impl BucketStates {
    pub fn new() -> BucketStates {
        BucketStates {
            round_states: BTreeMap::new(),
        }
    }
    pub fn ancillary_designation_is_available_for_this_round(
        &self,
        round: &u64,
        ancillary_designation: &str,
    ) -> bool {
        for n in 0..*round {
            let state = self.get_state(&n);
            if state
                .ancillary_designations
                .contains_key(ancillary_designation)
            {
                return false;
            }
        }
        true
    }

    pub fn get_state(&self, round: &u64) -> &BucketState {
        self.round_states.get(round).expect("Should exist.")
    }

    pub fn get_state_mut(&mut self, round: &u64) -> &mut BucketState {
        self.round_states.get_mut(round).expect("Should exist.")
    }

    pub fn selection_result(&self, round: &u64, participant: &Participant, selection: &Selection) {}
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BucketStates {
    round_states: BTreeMap<u64, BucketState>, //the state of each round in this bucket
}
