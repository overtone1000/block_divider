use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{
    basis::BlockDivisionBasis,
    participant::{self, ParticipantIndex},
    round::RoundIndex,
};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Selection {
    pub(crate) bucket_index: usize,
    pub(crate) ancillaries: BTreeSet<usize>, //this is where Black Butte will go but opens it to other possibilities
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Selections {
    state: BTreeMap<RoundIndex, BTreeMap<ParticipantIndex, BTreeSet<Selection>>>,
}

impl Selections {
    pub fn new(basis: &BlockDivisionBasis) -> Selections {
        let mut retval = Selections {
            state: BTreeMap::new(),
        };

        for round in 0..basis.get_selection_rounds().len() {
            let mut participant_selection_map: BTreeMap<ParticipantIndex, BTreeSet<Selection>> =
                BTreeMap::new();

            for participant in 0..basis.get_participant_definitions().len() {
                participant_selection_map.insert(participant, BTreeSet::new());
            }

            retval.state.insert(round, participant_selection_map);
        }

        retval
    }

    pub fn get(&self, round: &usize) -> Option<&BTreeMap<ParticipantIndex, BTreeSet<Selection>>> {
        self.state.get(round)
    }

    pub fn set(
        &mut self,
        round: usize,
        participant: ParticipantIndex,
        selections: BTreeSet<Selection>,
    ) {
        let round_selections = match self.state.entry(round) {
            std::collections::btree_map::Entry::Vacant(entry) => entry.insert(BTreeMap::new()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };
        round_selections.insert(participant, selections);
    }
}
