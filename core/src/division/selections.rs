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
    selections: BTreeMap<RoundIndex, BTreeMap<ParticipantIndex, BTreeSet<Selection>>>,
}

impl Selections {
    pub fn new(basis: &BlockDivisionBasis) -> Selections {
        let mut retval = Selections {
            selections: BTreeMap::new(),
        };

        for round in basis.get_selection_rounds().keys() {
            let mut participant_selection_map: BTreeMap<ParticipantIndex, BTreeSet<Selection>> =
                BTreeMap::new();

            for participant in basis.get_participant_definitions().keys() {
                participant_selection_map.insert(*participant, BTreeSet::new());
            }

            retval.selections.insert(*round, participant_selection_map);
        }

        retval
    }

    pub fn get(&self, round: &usize) -> Option<&BTreeMap<ParticipantIndex, BTreeSet<Selection>>> {
        self.selections.get(round)
    }

    pub fn set(
        &mut self,
        round: usize,
        participant: ParticipantIndex,
        selections: BTreeSet<Selection>,
    ) {
        let round_selections = match self.selections.entry(round) {
            std::collections::btree_map::Entry::Vacant(entry) => entry.insert(BTreeMap::new()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };
        round_selections.insert(participant, selections);
    }
}
