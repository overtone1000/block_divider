use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::participant::Participant;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
pub struct Selection {
    pub(crate) bucket_name: String,
    pub(crate) ancillaries: BTreeSet<String>, //this is where Black Butte will go but opens it to other possibilities
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Selections {
    selections: BTreeMap<u64, BTreeMap<Participant, BTreeSet<Selection>>>,
}

impl Selections {
    pub fn new() -> Selections {
        Selections {
            selections: BTreeMap::new(),
        }
    }

    pub fn get(&self, round: &u64) -> Option<&BTreeMap<Participant, BTreeSet<Selection>>> {
        self.selections.get(round)
    }

    pub fn set(&mut self, round: u64, participant: Participant, selections: BTreeSet<Selection>) {
        let round_selections = match self.selections.entry(round) {
            std::collections::btree_map::Entry::Vacant(entry) => entry.insert(BTreeMap::new()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };
        round_selections.insert(participant, selections);
    }
}
