use diesel::PgConnection;
use rand::prelude::*;

use std::{
    borrow::BorrowMut,
    collections::{BTreeMap, BTreeSet},
    error::Error,
    f32::consts::E,
    hash::{Hash, Hasher},
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use crate::db::division::PersistentDivision;

use super::{
    bucket::{self, BucketDef, BucketName, BucketState, BucketStates, Ranks},
    participant::Participant,
    round::{RoundIndex, RoundName},
    selections::Selection,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockDivisionBasis {
    pub bucket_definitions: BTreeMap<BucketName, BucketDef>,
    pub participant_round_picks: BTreeMap<Participant, BTreeMap<RoundIndex, u64>>,
    pub selection_rounds: Vec<RoundName>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BlockDivisionState {
    pub basis: BlockDivisionBasis,
    pub bucket_states: BTreeMap<BucketName, BucketStates>,
    pub selections: BTreeMap<u64, BTreeMap<Participant, BTreeSet<Selection>>>,
    pub current_open_round: u64,
}

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    round: u64,
    participant: Participant,
    selection: Selection,
}

//const STATE_CACHE_PATH: &str = "./state_cache/";

impl BlockDivisionState {
    pub fn create_empty(basis: &BlockDivisionBasis) -> BlockDivisionState {
        let mut retval = BlockDivisionState {
            basis: basis.clone(),
            bucket_states: BTreeMap::new(),
            selections: BTreeMap::new(),
            current_open_round: 0,
        };
        for (bucket_name, bucket_def) in &basis.bucket_definitions {
            retval
                .bucket_states
                .insert(bucket_name.to_string(), BucketStates::new());
        }
        retval
    }

    fn round_count(&self) -> u64 {
        self.basis.selection_rounds.len() as u64
    }

    pub fn input_selection(
        &mut self,
        conn: &mut PgConnection,
        participant: Participant,
        selections: BTreeSet<Selection>,
        round: u64,
    ) {
        let pick_count = self
            .basis
            .participant_round_picks
            .get(&participant)
            .expect("Should exist.")
            .get(&round)
            .expect("Should exist.");

        if (selections.len() as u64) > *pick_count {
            eprintln!(
                "Incorrect number of picks for {}. Ignoring selection input.",
                participant
            )
        } else if round != self.current_open_round {
            eprintln!(
                "Incorrect round {}. Open round is {}. Ignoring selection input.",
                round, self.current_open_round
            );
        } else {
            let round_selections = match self.selections.entry(round) {
                std::collections::btree_map::Entry::Vacant(entry) => entry.insert(BTreeMap::new()),
                std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
            };
            round_selections.insert(participant, selections);
            self.determine_designations_from_current_selections();
        }

        self.save_state(conn);
    }

    fn determine_designations_from_current_selections(&mut self) {
        for round in 0..self.round_count() {
            match self.selections.get(&round) {
                Some(participant_selections_map) => {
                    for (participant, selections) in participant_selections_map {
                        for selection in selections.iter() {
                            let bucket = &self
                                .bucket_states
                                .get_mut(&selection.bucket_name)
                                .expect(&format!(
                                    "Key {} should exist in bucket but wasn't found.",
                                    &selection.bucket_name
                                ));
                            self.attempt_selection(&round, &participant, &selection);
                        }
                    }
                }
                None => {}
            }
        }
    }

    fn save_state(&mut self, conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
        PersistentDivision::update(conn, self)
    }

    fn generate_ranks(&mut self) {
        let participant_count = self.basis.participant_round_picks.len() as u64;
        let mut initial_available_ranks: BTreeSet<u64> = BTreeSet::new();

        for n in 0..participant_count {
            initial_available_ranks.insert(n);
        }

        let mut rng: ThreadRng = thread_rng();

        println!("Iterating through {} rounds.", self.round_count());
        for round in 0..self.round_count() {
            println!("Round {}", round);
            for (bucket_name, bucket) in &mut self.bucket_states {
                let mut bucket_state_this_round: BTreeMap<Participant, u64> = BTreeMap::new();

                for participant in self.basis.participant_round_picks.keys() {
                    /*
                    println!("");
                    println!(
                        "Participant {} round {} bucket {}",
                        participant.to_string(),
                        round,
                        bucket_name
                    );

                    println!(
                        "Ranks already used for this bucket and round are {:?}",
                        (&bucket_state_this_round).values()
                    );
                    */

                    let mut available_ranks: BTreeSet<u64> = BTreeSet::new();
                    for r in initial_available_ranks.iter() {
                        available_ranks.insert(r.clone());
                    }
                    for r in (&bucket_state_this_round).values() {
                        available_ranks.remove(r);
                    }
                    let mut available_ranks_as_vec: Vec<u64> = Vec::new();
                    for r in available_ranks {
                        available_ranks_as_vec.push(r.to_owned());
                    }
                    /*
                    println!(
                        "This leaves the following available ranks for the randomizer {:?}",
                        (&available_ranks_as_vec)
                    );
                    */

                    let rank_index = rng.gen_range(0..available_ranks_as_vec.len());
                    let rank = available_ranks_as_vec
                        .get(rank_index)
                        .expect("Should exist.");

                    bucket_state_this_round.insert(participant.to_owned(), rank.to_owned());

                    /*
                    println!(
                        "Adding rank {} for participant {} for round {} in bucket {}",
                        rank,
                        participant.to_string(),
                        round,
                        bucket_name
                    );
                    println!("");
                    */
                }

                bucket.get_state_mut(&round).ranks = bucket_state_this_round;
            }
        }
    }

    pub(crate) fn pretty_print(&self) {
        let serialized = serde_json::to_string_pretty(self).expect("Should serialize.");
        println!("{}", serialized);
    }

    fn slots_available_this_round(&self, bucket_name: &str, round: &u64) -> u64 {
        let mut used_slots: u64 = 0;
        for current_round in 0..*round {
            used_slots += self
                .bucket_states
                .get(bucket_name)
                .expect("Bucket should exist.")
                .get_state(&current_round)
                .designations
                .len() as u64;
        }

        let available_slots = self
            .basis
            .bucket_definitions
            .get(bucket_name)
            .expect("Bucket should exist.")
            .available_slots;

        available_slots - used_slots
    }

    pub fn attempt_selection(
        &mut self,
        round: &u64,
        participant: &Participant,
        selection: &Selection,
    ) -> bool {
        let winners = {
            let bucket_states = self
                .bucket_states
                .get(&selection.bucket_name)
                .expect("Bucket should exist.");
            let round_state = bucket_states.get_state(&round);

            //Check ancillary designations first. If this participant loses any, reject the selection
            for ancillary_designation in &selection.ancillaries {
                if !bucket_states.ancillary_designation_is_available_for_this_round(
                    &round,
                    &ancillary_designation,
                ) {
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

            let slots_available = self.slots_available_this_round(&selection.bucket_name, round);

            let mut candidates: BTreeSet<Participant> =
                round_state.designations.clone().into_iter().collect();
            candidates.insert(participant.to_string());

            round_state.get_winners(&candidates, slots_available)
        };

        if winners.contains(participant) {
            //Update ancillaries and designations
            let bucket_states_mut = self
                .bucket_states
                .get_mut(&selection.bucket_name)
                .expect("Bucket should exist.");
            let round_state_mut = bucket_states_mut.get_state_mut(&round);

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
}

#[cfg(test)]
mod tests {
    use std::char::ParseCharError;

    use crate::db::{division::PersistentDivision, establish_connection};

    use super::*;

    const PARTICIPANT_A: &str = "Participant A";
    const PARTICIPANT_B: &str = "Participant B";
    const PARTICIPANT_C: &str = "Participant C";
    const BLACK_BUTTE: &str = "Black Butte";
    const NUMBER_OF_BUCKETS: u64 = 4;

    fn bucketname(i: u64) -> String {
        assert!(i >= 1 && i <= NUMBER_OF_BUCKETS);
        "Bucket ".to_string() + &i.to_string()
    }

    fn create_basis() -> BlockDivisionBasis {
        let mut buckets: BTreeMap<BucketName, BucketDef> = BTreeMap::new();

        for n in 1..NUMBER_OF_BUCKETS + 1 {
            buckets.insert(
                bucketname(n),
                BucketDef {
                    available_slots: 5,
                    available_ancillaries: BTreeSet::from([BLACK_BUTTE.to_string()]),
                },
            );
        }

        let rounds: Vec<RoundName> = [
            "Predesignation".to_string(),
            "Round 1".to_string(),
            "Round 2".to_string(),
            "Round 3".to_string(),
        ]
        .to_vec();

        let mut round_picks: BTreeMap<RoundIndex, u64> = BTreeMap::new();
        round_picks.insert(0, 3);
        for n in 1..rounds.len() as RoundIndex {
            round_picks.insert(n, 1);
        }

        let mut participants: BTreeMap<Participant, BTreeMap<RoundIndex, u64>> = BTreeMap::new();
        participants.insert(PARTICIPANT_A.to_string(), round_picks.clone());
        participants.insert(PARTICIPANT_B.to_string(), round_picks.clone());
        participants.insert(PARTICIPANT_C.to_string(), round_picks.clone());

        BlockDivisionBasis {
            bucket_definitions: buckets,
            participant_round_picks: participants,
            selection_rounds: rounds,
        }
    }

    #[test]
    fn block_division_cache_and_serialization_testing() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        let mut conn = establish_connection();
        let basis = create_basis();

        PersistentDivision::get_or_create(&mut conn, &basis, false) //create to test overwriting
            .expect("Should work.");
        let bds = PersistentDivision::get_or_create(&mut conn, &basis, false) //recreate to test ignoring
            .expect("Should work.");

        println!("----------------------");
        println!("Block Division State Serialization:");
        bds.pretty_print();
        println!("----------------------");
        println!("");

        let bds2 = PersistentDivision::get_or_create(&mut conn, &basis, true) //recreate to test equivalence
            .expect("Should work.");

        assert!(bds == bds2); //bds created from cache must be equal to the one that created the cache
        assert!(
            serde_json::to_string(&bds).expect("Should serialize.")
                == serde_json::to_string(&bds2).expect("Should serialize.")
        ); //serializations should also be equal.
    }

    #[test]
    fn selection_and_calculation() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        let mut conn = establish_connection();
        let basis = create_basis();

        let mut bds =
            PersistentDivision::get_or_create(&mut conn, &basis, false).expect("Should work.");

        let currentbucketname = bucketname(1);
        let currentround = 0;

        let mut selections_a: BTreeSet<Selection> = BTreeSet::new();
        let mut ancillaries_a: BTreeSet<String> = BTreeSet::new();
        ancillaries_a.insert(BLACK_BUTTE.to_string());
        selections_a.insert(Selection {
            bucket_name: currentbucketname.to_string(),
            ancillaries: ancillaries_a,
        });
        bds.input_selection(
            &mut conn,
            PARTICIPANT_A.to_string(),
            selections_a,
            currentround,
        );
        bds.determine_designations_from_current_selections();

        /*
        println!("----------------------");
        println!("Block Division State Serialization:");
        bds.pretty_print();
        println!("----------------------");
        println!("");
        */

        let pertinent_designations = &bds
            .bucket_states
            .get(&currentbucketname)
            .expect("Should exist.")
            .get_state(&currentround)
            .designations;

        let correctly_assigned = pertinent_designations.contains(PARTICIPANT_A);

        if !correctly_assigned {
            eprintln!(
                "Assignment failed. Designations for bucket {} and round {} are {:?}",
                currentbucketname, currentround, pertinent_designations
            );
        }
        assert!(correctly_assigned);
    }
}
