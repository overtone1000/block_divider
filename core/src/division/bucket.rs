use std::collections::{BTreeMap, BTreeSet, HashSet};

use serde::{Deserialize, Serialize};

use super::{block_division::BlockDivisionBasis, participant::Participant, selections::Selection};

pub type Designations = BTreeSet<Participant>; //Map containing selected participants. Keys are rounds of the selection.
pub type Ranks = BTreeMap<Participant, usize>;

pub type BucketName = String;
pub type AncillaryName = String;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct BucketDef {
    pub(crate) available_slots: usize, //How many participants can fit in this bucket in total
    pub(crate) available_ancillaries: BTreeSet<AncillaryName>, //What ancillaries are available to an individual participant in this bucket
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct BucketState {
    pub(crate) designations: Designations, //Who is in this bucket in each round (0 is predesignation, 1 is round 1, 2 is round 2, etc.)
    pub(crate) ancillary_designations: BTreeMap<BucketName, Participant>,
    pub(crate) ranks: Ranks, //Map containing the ranks for each participant for each potential round
}

impl BucketState {
    pub(crate) fn new() -> BucketState {
        BucketState {
            designations: Designations::new(),
            ancillary_designations: BTreeMap::new(),
            ranks: Ranks::new(),
        }
    }

    pub(crate) fn get_winners(
        &self,
        candidates: &BTreeSet<Participant>,
        winner_count: usize,
    ) -> BTreeSet<Participant> {
        let mut map: BTreeMap<&usize, Participant> = BTreeMap::new(); //automatically sorted by key
        for candidate in candidates {
            let rank = self.ranks.get(candidate).expect("Should exist.");
            map.insert(rank, candidate.to_string());
        }

        while map.len() as usize > winner_count {
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

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct RoundStates {
    round_states: BTreeMap<usize, BucketState>, //the state of each round in this bucket
}

impl RoundStates {
    pub fn new(basis: &BlockDivisionBasis) -> RoundStates {
        let mut retval = RoundStates {
            round_states: BTreeMap::new(),
        };
        for round in 0..basis.selection_rounds.len() {
            retval
                .round_states
                .insert(round as usize, BucketState::new());
        }
        retval
    }
    pub fn ancillary_designation_is_available_for_this_round(
        &self,
        round: &usize,
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

    pub fn get_state(&self, round: &usize) -> &BucketState {
        self.round_states.get(round).expect("Should exist.")
    }

    pub fn get_state_mut(&mut self, round: &usize) -> &mut BucketState {
        self.round_states.get_mut(round).expect("Should exist.")
    }

    pub fn selection_result(
        &self,
        round: &usize,
        participant: &Participant,
        selection: &Selection,
    ) {
        panic!("Not implemented.");
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BucketStates {
    bucket_states: BTreeMap<BucketName, RoundStates>,
}

impl BucketStates {
    pub fn new(basis: &BlockDivisionBasis) -> BucketStates {
        let mut retval = BucketStates {
            bucket_states: BTreeMap::new(),
        };
        for (bucket_name, bucket_def) in &basis.bucket_definitions {
            retval
                .bucket_states
                .insert(bucket_name.to_string(), RoundStates::new(basis));
        }
        retval
    }

    pub fn get(&self, bucket_name: &str) -> Option<&RoundStates> {
        self.bucket_states.get(bucket_name)
    }

    pub fn get_mut(&mut self, bucket_name: &str) -> Option<&mut RoundStates> {
        self.bucket_states.get_mut(bucket_name)
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, std::string::String, RoundStates> {
        self.bucket_states.iter()
    }

    pub fn iter_mut(
        &mut self,
    ) -> std::collections::btree_map::IterMut<'_, std::string::String, RoundStates> {
        self.bucket_states.iter_mut()
    }
}
