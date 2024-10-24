use std::collections::{BTreeMap, BTreeSet, HashSet};

use serde::{Deserialize, Serialize};

use super::{
    basis::BlockDivisionBasis, participant::ParticipantIndex, round::RoundIndex,
    selections::Selection,
};

pub type Designations = BTreeSet<ParticipantIndex>; //Map containing selected participants. Keys are rounds of the selection.
pub type Ranks = BTreeMap<ParticipantIndex, usize>;

pub type BucketIndex = usize;
pub type AncillaryIndex = usize;
pub type AncillaryName = String;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct BucketDef {
    pub(crate) name: String,
    pub(crate) available_slots: usize, //How many participants can fit in this bucket in total
    pub(crate) available_ancillaries: Vec<AncillaryName>, //What ancillaries are available to an individual participant in this bucket
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq, Clone)]
pub struct BucketState {
    pub(crate) designations: Designations, //Who is in this bucket in each round (0 is predesignation, 1 is round 1, 2 is round 2, etc.)
    pub(crate) ancillary_designations: BTreeMap<AncillaryIndex, ParticipantIndex>,
    pub(crate) ranks: Option<Ranks>, //Map containing the ranks for each participant for each potential round, make an option to allow frontend censoring
}

impl BucketState {
    pub(crate) fn new() -> BucketState {
        BucketState {
            designations: Designations::new(),
            ancillary_designations: BTreeMap::new(),
            ranks: Some(Ranks::new()),
        }
    }

    pub(crate) fn get_winners(
        &self,
        candidates: &BTreeSet<ParticipantIndex>,
        winner_count: usize,
    ) -> BTreeSet<ParticipantIndex> {
        let mut map: BTreeMap<&usize, ParticipantIndex> = BTreeMap::new(); //automatically sorted by key
        for candidate in candidates {
            let rank = self.get_rank(candidate);
            map.insert(rank, *candidate);
        }

        while map.len() as usize > winner_count {
            map.pop_last();
        }

        let mut retval: BTreeSet<ParticipantIndex> = BTreeSet::new();
        for participant in map.values() {
            retval.insert(*participant);
        }
        retval
    }

    pub(crate) fn get_rank(&self, participant: &ParticipantIndex) -> &ParticipantIndex {
        self.ranks
            .as_ref()
            .expect("Should always be some. None is only used for frontend censoring")
            .get(participant)
            .expect("Should always exist.")
    }

    pub(crate) fn is_winner(
        &self,
        contender: &ParticipantIndex,
        opponent: &ParticipantIndex,
    ) -> bool {
        if self.get_rank(contender) > self.get_rank(opponent) {
            false
        } else {
            true
        }
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct RoundStates {
    round_states: Vec<BucketState>, //the state of each round in this bucket
}

impl RoundStates {
    pub fn new(basis: &BlockDivisionBasis) -> RoundStates {
        let mut retval = RoundStates {
            round_states: Vec::new(),
        };
        for round in 0..basis.get_selection_rounds().len() {
            retval.round_states.insert(round, BucketState::new());
        }
        retval
    }

    pub fn ancillary_designation_is_available_for_this_round(
        &self,
        round: &RoundIndex,
        ancillary_designation: &AncillaryIndex,
    ) -> bool {
        for n in 0..self.round_states.len() {
            if n == *round {
                break;
            }
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
        self.round_states.get(*round).expect("Should exist.")
    }

    pub fn get_state_mut(&mut self, round: &usize) -> &mut BucketState {
        self.round_states.get_mut(*round).expect("Should exist.")
    }

    pub fn get_states(&self) -> &Vec<BucketState> {
        &self.round_states
    }

    pub fn get_states_mut(&mut self) -> &mut Vec<BucketState> {
        &mut self.round_states
    }

    pub fn selection_result(
        &self,
        round: &RoundIndex,
        participant: &ParticipantIndex,
        selection: &Selection,
    ) {
        panic!("Not implemented.");
    }
}

pub type BucketStates = Vec<RoundStates>;
