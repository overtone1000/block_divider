use chrono::round;
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
    basis::BlockDivisionBasis,
    bucket::{
        self, BucketDef, BucketIndex, BucketName, BucketState, BucketStates, Ranks, RoundStates,
    },
    participant::{ParticipantDef, ParticipantIndex},
    round::{RoundIndex, RoundName},
    selections::{Selection, Selections},
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BlockDivisionState {
    pub basis: BlockDivisionBasis,
    pub bucket_states: BucketStates,
    pub selections: Selections,
    pub current_open_round: Option<RoundIndex>,
}

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    round: usize,
    participant: ParticipantIndex,
    selection: Selection,
}

//const STATE_CACHE_PATH: &str = "./state_cache/";

impl BlockDivisionState {
    pub fn new(basis: &BlockDivisionBasis) -> BlockDivisionState {
        let mut bucket_states: BucketStates = BTreeMap::new();
        for bucket_index in basis.get_bucket_definitions().keys() {
            bucket_states.insert(*bucket_index, RoundStates::new(basis));
        }

        let mut retval = BlockDivisionState {
            basis: basis.clone(),
            bucket_states: bucket_states,
            selections: Selections::new(basis),
            current_open_round: None,
        };

        retval.generate_ranks(); //Only generate ranks here. This should only happen once per basis.

        retval
    }

    pub fn input_selection(
        &mut self,
        conn: &mut PgConnection,
        participant_index: ParticipantIndex,
        selections: BTreeSet<Selection>,
        round: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pick_count = self
            .basis
            .get_participant_definitions()
            .get(&participant_index)
            .expect("Participant should exist.")
            .get_round_picks_allowed()
            .get(&round)
            .expect("Round should exist.");

        match self.current_open_round {
            Some(current_open_round) => {
                if selections.len() > (*pick_count as usize) {
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!(
                            "Incorrect number of picks for {}. Ignoring selection input.",
                            participant_index
                        ),
                    )))
                } else if round != current_open_round {
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!(
                            "Incorrect round {}. Open round is {}. Ignoring selection input.",
                            round, current_open_round
                        ),
                    )))
                } else {
                    self.selections.set(round, participant_index, selections);
                    self.determine_designations_from_current_selections();
                    match self.save_state(conn) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("Couldn't save state."),
                        ))),
                    }
                }
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Selections are closed."),
            ))),
        }
    }

    fn determine_designations_from_current_selections(&mut self) {
        let mut selections_to_attempt: Vec<(usize, ParticipantIndex, Selection)> = Vec::new();

        for round in self.basis.get_selection_rounds().keys() {
            match self.selections.get(&round) {
                Some(participant_selections_map) => {
                    for (participant, selections) in participant_selections_map {
                        for selection in selections.iter() {
                            selections_to_attempt.push((
                                *round,
                                participant.clone(),
                                selection.clone(),
                            ));
                        }
                    }
                }
                None => {}
            };
        }

        for (round, participant, selection) in selections_to_attempt {
            self.attempt_selection(&round, &participant, &selection);
        }
    }

    fn save_state(&mut self, conn: &mut PgConnection) -> Result<(), Box<dyn std::error::Error>> {
        PersistentDivision::update(conn, self)
    }

    fn generate_ranks(&mut self) {
        let mut initial_available_ranks: BTreeSet<usize> = BTreeSet::new();

        for n in 1..self.basis.get_participant_definitions().len() + 1 {
            initial_available_ranks.insert(n);
        }

        let mut rng: ThreadRng = thread_rng();

        println!(
            "Iterating through {} rounds.",
            self.basis.get_selection_rounds().keys().len()
        );
        for round in self.basis.get_selection_rounds().keys() {
            println!("Round {}", round);
            for (_bucket_name, bucket) in self.bucket_states.iter_mut() {
                let mut bucket_state_this_round: BTreeMap<ParticipantIndex, usize> =
                    BTreeMap::new();

                for participant in self.basis.get_participant_definitions().keys() {
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

                    let mut available_ranks: BTreeSet<usize> = BTreeSet::new();
                    for r in initial_available_ranks.iter() {
                        available_ranks.insert(r.clone());
                    }
                    for r in (&bucket_state_this_round).values() {
                        available_ranks.remove(r);
                    }
                    let mut available_ranks_as_vec: Vec<usize> = Vec::new();
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

    fn slots_available_this_round(&self, bucket_index: &usize, round: &usize) -> usize {
        let mut used_slots: usize = 0;
        for current_round in self.basis.get_selection_rounds().keys() {
            used_slots += self
                .bucket_states
                .get(bucket_index)
                .expect("Bucket should exist.")
                .get_state(&current_round)
                .designations
                .len() as usize;
        }

        let available_slots = self
            .basis
            .get_bucket_definitions()
            .get(bucket_index)
            .expect("Bucket should exist.")
            .available_slots;

        available_slots - used_slots
    }

    pub fn attempt_selection(
        &mut self,
        round: &usize,
        participant: &ParticipantIndex,
        selection: &Selection,
    ) -> bool {
        let winners = {
            let bucket_states = self
                .bucket_states
                .get(&selection.bucket_index)
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

            let slots_available = self.slots_available_this_round(&selection.bucket_index, round);

            let mut candidates: BTreeSet<ParticipantIndex> =
                round_state.designations.clone().into_iter().collect();
            candidates.insert(*participant);

            round_state.get_winners(&candidates, slots_available)
        };

        if winners.contains(participant) {
            //Update ancillaries and designations
            let bucket_states_mut = self
                .bucket_states
                .get_mut(&selection.bucket_index)
                .expect("Bucket should exist.");
            let round_state_mut = bucket_states_mut.get_state_mut(&round);

            round_state_mut.designations = winners;

            for ancillary_designation in &selection.ancillaries {
                round_state_mut
                    .ancillary_designations
                    .insert(*ancillary_designation, *participant);
            }

            true
        } else {
            false
        }
    }

    pub fn set_open_round(
        &mut self,
        round_index: Option<usize>,
        conn: &mut PgConnection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match round_index {
            Some(round_index) => {
                let mut contains_key = false;
                for key in self.basis.get_selection_rounds().keys() {
                    if key == &round_index {
                        self.current_open_round = Some(round_index);
                        contains_key = true;
                        break;
                    }
                }
                if !contains_key {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Invalid round index {}.", round_index),
                    )));
                }
            }
            None => {}
        };

        self.current_open_round = round_index;
        self.save_state(conn)
    }
}

#[cfg(test)]
mod tests {
    use std::char::ParseCharError;

    use bucket::AncillaryIndex;

    use crate::db::{division::PersistentDivision, establish_connection};

    use super::*;

    const PARTICIPANT_A: (usize, &str) = (2, "Participant A");
    const PARTICIPANT_B: (usize, &str) = (18, "Participant A");
    const PARTICIPANT_C: (usize, &str) = (47, "Participant A");
    const BLACK_BUTTE: (usize, &str) = (5, "Black Butte");
    const BUCKET_INDICES: [usize; 6] = [1, 5, 18, 2, 7, 77];

    const ROUND_1: (usize, &str) = (2, "Predesignation");
    const ROUND_2: (usize, &str) = (6, "Round 1");
    const ROUND_3: (usize, &str) = (988, "Round 2");
    const ROUND_4: (usize, &str) = (5, "Round 3");

    fn bucketname(i: usize) -> String {
        "Bucket ".to_string() + &i.to_string()
    }

    fn create_basis() -> BlockDivisionBasis {
        let mut buckets: BTreeMap<BucketIndex, BucketDef> = BTreeMap::new();

        for n in BUCKET_INDICES {
            buckets.insert(
                n,
                BucketDef {
                    name: bucketname(n),
                    available_slots: 5,
                    available_ancillaries: BTreeMap::from([(0, BLACK_BUTTE.1.to_string())]),
                },
            );
        }

        let rounds: BTreeMap<RoundIndex, RoundName> = BTreeMap::from([
            (ROUND_1.0, ROUND_1.1.to_string()),
            (ROUND_2.0, ROUND_2.1.to_string()),
            (ROUND_3.0, ROUND_3.1.to_string()),
            (ROUND_4.0, ROUND_4.1.to_string()),
        ]);

        let mut round_picks: BTreeMap<RoundIndex, u64> = BTreeMap::new();
        round_picks.insert(0, 3);
        for n in rounds.keys() {
            round_picks.insert(*n, 1);
        }

        let mut participants: BTreeMap<ParticipantIndex, ParticipantDef> = BTreeMap::new();

        for participant in [PARTICIPANT_A, PARTICIPANT_B, PARTICIPANT_C] {
            participants.insert(
                participant.0,
                ParticipantDef::create(participant.1.to_string(), round_picks.clone()),
            );
        }

        BlockDivisionBasis::create(
            "Test Block Division".to_string(),
            buckets,
            participants,
            rounds,
        )
    }

    #[test]
    fn block_division_cache_and_serialization_testing() {
        dotenvy::dotenv().expect("Couldn't load environment variables.");
        let mut conn = establish_connection();
        let basis = create_basis();

        match PersistentDivision::delete_division_from_basis(&mut conn, &basis) //Delete any remnant
        {
            Ok(_)=>{},
            Err(_)=>{println!("Couldn't delete, but this may not be an error.");}
        };

        PersistentDivision::get_or_create(&mut conn, &basis) //create to test overwriting
            .expect("Should work.");
        let delete_count =
            PersistentDivision::delete_division_from_basis(&mut conn, &basis).expect("Should work"); //Delete just created, should have a result
        assert!(delete_count == 1);

        let bds = PersistentDivision::get_or_create(&mut conn, &basis) //recreate to test ignoring
            .expect("Should work.");

        println!("----------------------");
        println!("Block Division State Serialization:");
        bds.pretty_print();
        println!("----------------------");
        println!("");

        let bds2 = PersistentDivision::get_or_create(&mut conn, &basis) //recreate to test equivalence
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

        match PersistentDivision::delete_division_from_basis(&mut conn, &basis) //Delete any remnant
        {
            Ok(_)=>{},
            Err(_)=>{println!("Couldn't delete, but this may not be an error.");}
        }
        let mut bds = PersistentDivision::get_or_create(&mut conn, &basis).expect("Should work.");

        let participant_index = PARTICIPANT_A.0;

        let current_bucket_index = basis
            .get_bucket_definitions()
            .keys()
            .next()
            .expect("Should have a first value.");
        let currentbucketname = bucketname(*current_bucket_index);

        let current_round_index = basis
            .get_selection_rounds()
            .keys()
            .next()
            .expect("Should have a first value.");

        let mut selections_a: BTreeSet<Selection> = BTreeSet::new();
        let mut ancillaries_a: BTreeSet<AncillaryIndex> = BTreeSet::new();
        ancillaries_a.insert(BLACK_BUTTE.0);

        selections_a.insert(Selection {
            bucket_index: *current_bucket_index,
            ancillaries: ancillaries_a,
        });

        bds.set_open_round(Some(*current_round_index), &mut conn)
            .expect("Couldn't set open round.");

        bds.input_selection(
            &mut conn,
            participant_index,
            selections_a,
            *current_round_index,
        )
        .expect("Should be able to input selection.");
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
            .get(current_bucket_index)
            .expect("Should exist.")
            .get_state(current_round_index)
            .designations;

        let correctly_assigned = pertinent_designations.contains(&participant_index);

        if !correctly_assigned {
            eprintln!(
                "Assignment failed. Designations for bucket {} and round {} are {:?}",
                currentbucketname, current_round_index, pertinent_designations
            );
        }
        assert!(correctly_assigned);
    }
}
