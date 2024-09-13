use std::collections::{BTreeMap, BTreeSet, HashSet};

use serde::{Deserialize, Serialize};

use super::{participant::Participant, selections::Selection};

pub type Designations = BTreeSet<Participant>; //Map containing selected participants. Keys are rounds of the selection.
pub type Ranks = BTreeMap<Participant, u64>;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct BucketDef {
    pub(crate) available_slots: u64, //How many participants can fit in this bucket in total
    pub(crate) available_ancillaries: BTreeSet<String>, //What ancillaries are available to an individual participant in this bucket
}

#[derive(Deserialize, Serialize, Debug, Default, PartialEq, Eq)]
pub struct BucketState {
    pub(crate) designations: Designations, //Who is in this bucket in each round (0 is predesignation, 1 is round 1, 2 is round 2, etc.)
    pub(crate) ancillary_designations: BTreeMap<String, Participant>,
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

impl Bucket {
    pub fn attempt_selection(
        &mut self,
        round: &u64,
        participant: &Participant,
        selection: &Selection,
    ) -> bool {
        let round_state = self.get_state(&round);

        //Check ancillary designations first. If this participant loses any, reject the selection
        for ancillary_designation in &selection.ancillaries {
            if !self
                .ancillary_designation_is_available_for_this_round(&round, &ancillary_designation)
            {
                //Can't get ancillary, so selection is denied
                return false;
            }

            match round_state
                .ancillary_designations
                .get(ancillary_designation)
            {
                Some(current_ancillary_designee) => {
                    if !round_state.is_winner(participant, current_ancillary_designee) {
                        //Can't get ancillary, so selection is denied
                        return false;
                    }
                }
                None => {}
            }
        }

        let slots_available = self.slots_available_this_round(round);

        let mut candidates: BTreeSet<Participant> =
            round_state.designations.clone().into_iter().collect();
        candidates.insert(participant.to_string());

        let winners = round_state.get_winners(&candidates, slots_available);

        if winners.contains(participant) {
            //Update ancillaries and designations
            let round_state_mut = self.get_state_mut(round);
            round_state_mut.designations = winners;

            for ancillary_designation in &selection.ancillaries {
                round_state_mut
                    .ancillary_designations
                    .insert(ancillary_designation.to_string(), participant.to_string());
            }

            true
        } else {
            false
        }
    }

    fn ancillary_designation_is_available_for_this_round(
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

    fn slots_available_this_round(&self, round: &u64) -> u64 {
        let mut used_slots: u64 = 0;
        for current_round in 0..*round {
            used_slots += self.get_state(&current_round).designations.len() as u64;
        }
        self.definition.available_slots - used_slots
    }

    fn get_state(&self, round: &u64) -> &BucketState {
        self.state.get(round).expect("Should exist.")
    }

    fn get_state_mut(&mut self, round: &u64) -> &mut BucketState {
        self.state.get_mut(round).expect("Should exist.")
    }

    fn selection_result(&self, round: &u64, participant: &Participant, selection: &Selection) {}
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct Bucket {
    pub(crate) definition: BucketDef,
    pub(crate) state: BTreeMap<u64, BucketState>, //the state of each round in this bucket
}
