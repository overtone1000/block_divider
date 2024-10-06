use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{
    block_division::BlockDivisionBasis,
    participant::{self, Participant},
};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Selection {
    pub(crate) bucket_name: String,
    pub(crate) ancillaries: BTreeSet<String>, //this is where Black Butte will go but opens it to other possibilities
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Selections {
    selections: BTreeMap<usize, BTreeMap<Participant, BTreeSet<Selection>>>,
}

impl Selections {
    pub fn new(basis: &BlockDivisionBasis) -> Selections {
        let mut retval = Selections {
            selections: BTreeMap::new(),
        };

        for round in 0..basis.selection_rounds.len() {
            let mut participant_selection_map: BTreeMap<Participant, BTreeSet<Selection>> =
                BTreeMap::new();

            for participant in basis.participant_round_picks.keys() {
                participant_selection_map.insert(participant.to_string(), BTreeSet::new());
            }

            retval.selections.insert(round, participant_selection_map);
        }

        retval
    }

    pub fn get(&self, round: &usize) -> Option<&BTreeMap<Participant, BTreeSet<Selection>>> {
        self.selections.get(round)
    }

    pub fn set(&mut self, round: usize, participant: Participant, selections: BTreeSet<Selection>) {
        let round_selections = match self.selections.entry(round) {
            std::collections::btree_map::Entry::Vacant(entry) => entry.insert(BTreeMap::new()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };
        round_selections.insert(participant, selections);
    }
}
