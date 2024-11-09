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
    thread::current,
};

use serde::{Deserialize, Serialize};

use crate::db::division::PersistentDivision;

use super::{
    basis::BlockDivisionBasis,
    bucket::{self, BucketDef, BucketIndex, BucketState, BucketStates, Ranks, RoundStates},
    participant::{ParticipantDef, ParticipantIndex},
    round::{RoundIndex, RoundName},
    selections::{Selection, SelectionResult, Selections},
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockDivisionState {
    basis: BlockDivisionBasis,
    bucket_states: BucketStates,
    selections: Selections,
    current_open_round: Option<RoundIndex>,
}

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    round: usize,
    participant: ParticipantIndex,
    selection: Selection,
}

//const STATE_CACHE_PATH: &str = "./state_cache/";

impl BlockDivisionState {
    pub fn get_basis(&self) -> &BlockDivisionBasis {
        &self.basis
    }

    pub fn get_current_open_round(&self) -> &Option<RoundIndex> {
        &self.current_open_round
    }

    pub fn get_bucket_states_mut(&mut self) -> &mut BucketStates {
        &mut self.bucket_states
    }

    pub fn new(basis: &BlockDivisionBasis) -> BlockDivisionState {
        let mut bucket_states: BucketStates = Vec::new();
        for bucket_index in 0..basis.get_bucket_definitions().len() {
            bucket_states.insert(bucket_index, RoundStates::new(basis));
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

    pub fn set_selections_for_current_round(
        conn: &mut PgConnection,
        state_id: String,
        participant_index: ParticipantIndex,
        selections: Vec<Option<Selection>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match PersistentDivision::get_from_id(conn, &state_id) {
            Some(pd) => match pd.as_state() {
                Ok(mut state) => match state.current_open_round {
                    Some(current_open_round) => {
                        let pick_count = state
                            .basis
                            .get_participant_definitions()
                            .get(participant_index)
                            .expect("Participant should exist.")
                            .get_round_picks_allowed()
                            .get(current_open_round)
                            .expect("Round should exist.");

                        if selections.len() != (*pick_count as usize) {
                            Err(Box::new(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!(
                                    "Incorrect number of picks for {}. Ignoring selection input.",
                                    participant_index
                                ),
                            )))
                        } else {
                            state
                                .selections
                                .set(current_open_round, participant_index, selections);

                            state.determine_designations_from_current_selections();

                            state.save_state(state_id, conn)
                        }
                    }
                    None => Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        format!("Selections are closed."),
                    ))),
                },
                Err(e) => Err(e),
            },
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No state with id {}", state_id),
            ))),
        }
    }

    fn determine_designations_from_current_selections(&mut self) {
        struct SelectionInstance {
            rank: usize,
            round: RoundIndex,
            participant: ParticipantIndex,
            selection: Selection,
        }

        let mut selections_to_attempt: Vec<SelectionInstance> = Vec::new();

        for round in 0..self.basis.get_selection_rounds().len() {
            println!("Getting selections for round {}", round);
            match self.selections.get(&round) {
                Some(participant_selections_map) => {
                    for (participant, selections) in participant_selections_map {
                        for selection in selections.iter() {
                            match selection {
                                Some(selection) => {
                                    let rank = self
                                        .bucket_states
                                        .get(selection.bucket_index)
                                        .expect("Should exist.")
                                        .get_state(&round)
                                        .get_rank(participant);

                                    selections_to_attempt.push(SelectionInstance {
                                        rank: *rank,
                                        round: round,
                                        participant: participant.clone(),
                                        selection: selection.clone(),
                                    });
                                }
                                None => { //Do Nothing}
                                }
                            }
                        }
                    }
                }
                None => {
                    eprintln!("Empty selections value.");
                }
            };
        }

        //Clear all designations
        for state in &mut self.bucket_states {
            for state in state.get_states_mut() {
                state.designations.clear();
                state.ancillary_designations.clear();
            }
        }

        //Sort selections to attempt in order of highest rank to lowest rank
        selections_to_attempt
            .sort_by(|a: &SelectionInstance, b: &SelectionInstance| a.rank.cmp(&b.rank));
        for mut selection_instance in selections_to_attempt {
            println!(
                "Selection rank: {}, participant {}, bucket {}",
                selection_instance.rank,
                selection_instance.participant,
                selection_instance.selection.bucket_index
            );
            let result = self.attempt_selection(
                &selection_instance.round,
                &selection_instance.participant,
                &selection_instance.selection,
            );
            selection_instance.selection.state = Some(result);
        }

        //Caller must save state so selection results persist.
    }

    fn save_state(
        &mut self,
        id: String,
        conn: &mut PgConnection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        PersistentDivision::update(conn, id, self)
    }

    fn generate_ranks(&mut self) {
        let mut initial_available_ranks: BTreeSet<usize> = BTreeSet::new();

        for n in 1..self.basis.get_participant_definitions().len() + 1 {
            initial_available_ranks.insert(n);
        }

        let mut rng: ThreadRng = thread_rng();

        println!(
            "Iterating through {} rounds.",
            self.basis.get_selection_rounds().len()
        );
        for round in 0..self.basis.get_selection_rounds().len() {
            println!("Round {}", round);
            for bucket in self.bucket_states.iter_mut() {
                let mut bucket_state_this_round: BTreeMap<ParticipantIndex, usize> =
                    BTreeMap::new();

                for participant in 0..self.basis.get_participant_definitions().len() {
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

                bucket.get_state_mut(&round).ranks = Some(bucket_state_this_round);
            }
        }
    }

    pub(crate) fn pretty_print(&self) {
        let serialized = serde_json::to_string_pretty(self).expect("Should serialize.");
        println!("{}", serialized);
    }

    fn slots_available_this_round(&self, bucket_index: &usize, current_round: &usize) -> usize {
        let mut used_slots: usize = 0;

        if current_round > &0 {
            for previous_round in 0..current_round - 1 {
                used_slots += self
                    .bucket_states
                    .get(*bucket_index)
                    .expect("Bucket should exist.")
                    .get_state(&previous_round)
                    .designations
                    .len() as usize;
            }
        }

        let available_slots = self
            .basis
            .get_bucket_definitions()
            .get(*bucket_index)
            .expect("Bucket should exist.")
            .available_slots;

        available_slots - used_slots
    }

    pub fn attempt_selection(
        &mut self,
        round: &usize,
        participant: &ParticipantIndex,
        selection: &Selection,
    ) -> SelectionResult {
        let winners = {
            let bucket_states = self
                .bucket_states
                .get(selection.bucket_index)
                .expect("Bucket should exist.");
            let round_state = bucket_states.get_state(&round);

            //Check ancillary designations first. If this participant loses any, reject the selection
            let mut unavailable_ancillaries: Vec<usize> = Vec::new();
            for ancillary_designation in &selection.ancillaries {
                if !bucket_states.ancillary_designation_is_available_for_this_round(
                    &round,
                    &ancillary_designation,
                ) {
                    //Can't get ancillary, so selection is denied
                    unavailable_ancillaries.push(*ancillary_designation);
                }

                match round_state
                    .ancillary_designations
                    .get(ancillary_designation)
                {
                    Some(current_ancillary_designee) => {
                        if !round_state.is_winner(participant, current_ancillary_designee) {
                            //Can't get ancillary, so selection is denied
                            println!(
                                "{} beat {} for ancillary {}",
                                current_ancillary_designee, participant, ancillary_designation
                            );
                            unavailable_ancillaries.push(*ancillary_designation);
                        }
                    }
                    None => {}
                }
            }
            if unavailable_ancillaries.len() > 0 {
                return SelectionResult::RejectedAncillaryUnavailable(unavailable_ancillaries);
            }

            let slots_available = self.slots_available_this_round(&selection.bucket_index, round);
            if slots_available <= 0 {
                return SelectionResult::RejectedNoSelectionsThisRound;
            }

            let mut candidates: BTreeSet<ParticipantIndex> =
                round_state.designations.clone().into_iter().collect();
            candidates.insert(*participant);

            round_state.get_winners(&candidates, slots_available)
        };

        if winners.contains(participant) {
            //Update ancillaries and designations
            let bucket_states_mut = self
                .bucket_states
                .get_mut(selection.bucket_index)
                .expect("Bucket should exist.");
            let round_state_mut = bucket_states_mut.get_state_mut(&round);

            round_state_mut.designations = winners;

            for ancillary_designation in &selection.ancillaries {
                round_state_mut
                    .ancillary_designations
                    .insert(*ancillary_designation, *participant);
            }

            SelectionResult::Confirmed
        } else {
            SelectionResult::RejectedOutranked
        }
    }

    pub fn set_open_round(
        conn: &mut PgConnection,
        state_id: String,
        round_index: Option<usize>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match PersistentDivision::get_from_id(conn, &state_id) {
            Some(pd) => match pd.as_state() {
                Ok(mut state) => {
                    match round_index {
                        Some(round_index) => {
                            let mut contains_key = false;
                            for key in 0..state.basis.get_selection_rounds().len() {
                                if key == round_index {
                                    state.current_open_round = Some(round_index);
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

                    state.current_open_round = round_index;
                    state.save_state(state_id, conn)
                }
                Err(e) => Err(e),
            },
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("No state with id {}", state_id),
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use bucket::AncillaryIndex;

    use crate::db::{division::PersistentDivision, establish_connection};

    use super::*;

    const PARTICIPANT_A: (usize, &str, &str) = (0, "Participant A", "testing_a@autoscheda.com");
    const PARTICIPANT_B: (usize, &str, &str) = (1, "Participant B", "testing_b@autoscheda.com");
    const PARTICIPANT_C: (usize, &str, &str) = (2, "Participant C", "testing_c@autoscheda.com");
    const BLACK_BUTTE: (usize, &str) = (0, "Black Butte");
    const BUCKET_INDICES: [usize; 6] = [0, 1, 2, 3, 4, 5];

    const ROUND_1: (usize, &str) = (0, "Predesignation");
    const ROUND_2: (usize, &str) = (1, "Round 1");
    const ROUND_3: (usize, &str) = (2, "Round 2");
    const ROUND_4: (usize, &str) = (3, "Round 3");

    const PICKS_PER_ROUND: usize = 1;

    fn bucketname(i: usize) -> String {
        "Bucket ".to_string() + &i.to_string()
    }

    fn create_basis() -> BlockDivisionBasis {
        let mut buckets: Vec<BucketDef> = Vec::new();

        for n in BUCKET_INDICES {
            buckets.insert(
                n,
                BucketDef {
                    name: bucketname(n),
                    available_slots: 5,
                    available_ancillaries: Vec::from([(BLACK_BUTTE.1.to_string())]),
                },
            );
        }

        let rounds: Vec<RoundName> = Vec::from([
            ROUND_1.1.to_string(),
            ROUND_2.1.to_string(),
            ROUND_3.1.to_string(),
            ROUND_4.1.to_string(),
        ]);

        let mut round_picks: Vec<usize> = Vec::new();
        for n in 0..rounds.len() {
            round_picks.insert(n, PICKS_PER_ROUND);
        }

        let mut participants: Vec<ParticipantDef> = Vec::new();

        for participant in [PARTICIPANT_A, PARTICIPANT_B, PARTICIPANT_C] {
            participants.insert(
                participant.0,
                ParticipantDef::create(
                    participant.1.to_string(),
                    participant.2.to_string(),
                    round_picks.clone(),
                ),
            );
        }

        BlockDivisionBasis::create(buckets, participants, rounds)
    }

    #[test]
    fn block_division_cache_and_serialization_testing() {
        dotenvy::dotenv().expect("Couldn't load environment variables for testing.");
        let mut conn = establish_connection();
        let basis = create_basis();

        let id1 = "Test Block Division 1";
        let id2 = "Test Block Division 2";

        match PersistentDivision::delete_division(&mut conn, id1.to_string()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Couldn't delete pre-existing pd, but this may not be an error.");
            }
        }

        match PersistentDivision::delete_division(&mut conn, id2.to_string()) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Couldn't delete pre-existing pd, but this may not be an error.");
            }
        }

        let pd = PersistentDivision::new(&mut conn, id1.to_string(), &basis) //create to test overwriting
            .expect("Should work.");
        let delete_count =
            PersistentDivision::delete_division(&mut conn, pd.get_id()).expect("Should work"); //Delete just created, should have a result
        assert!(delete_count == 1);

        let pd = PersistentDivision::new(&mut conn, id1.to_string(), &basis) //recreate to test ignoring
            .expect("Should work.");
        let bds = pd.as_state().expect("Should be a state.");

        println!("----------------------");
        println!("Block Division State Serialization:");
        bds.pretty_print();
        println!("----------------------");
        println!("");

        let pd2 = PersistentDivision::new(&mut conn, id2.to_string(), &basis) //recreate to test equivalence
            .expect("Should work.");
        let bds2 = pd2.as_state().expect("Should be a state.");

        assert!(pd != pd2); //Should have different ids.
        assert!(bds.basis == bds2.basis); //But basis should be identical
        assert!(bds.current_open_round == bds2.current_open_round); //And current round as well
    }

    #[test]
    fn selection_and_calculation() {
        dotenvy::dotenv().expect("Couldn't load environment variables for testing.");
        let mut conn = establish_connection();
        let basis = create_basis();

        let id1 = "Test Block Division 3";
        match PersistentDivision::delete_division(&mut conn, id1.to_string()) {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Couldn't delete pre-existing pd, but this may not be an error.");
            }
        }

        PersistentDivision::new(&mut conn, id1.to_string(), &basis).expect("Should work."); //Test overwriting

        let participant_index = PARTICIPANT_A.0;

        let current_bucket_index = BUCKET_INDICES[0];
        let currentbucketname = bucketname(current_bucket_index);

        let current_round_index = ROUND_1.0;

        let mut selections_a: Vec<Option<Selection>> = Vec::new();
        let mut ancillaries_a: BTreeSet<AncillaryIndex> = BTreeSet::new();
        ancillaries_a.insert(BLACK_BUTTE.0);

        for _n in 0..PICKS_PER_ROUND {
            selections_a.push(Some(Selection {
                bucket_index: current_bucket_index,
                ancillaries: ancillaries_a.clone(),
                state: None,
            }));
        }

        BlockDivisionState::set_open_round(&mut conn, id1.to_string(), Some(current_round_index))
            .expect("Couldn't set open round.");

        BlockDivisionState::set_selections_for_current_round(
            &mut conn,
            id1.to_string(),
            participant_index,
            selections_a,
        )
        .expect("Should be able to input selection.");

        let bds = PersistentDivision::get_from_id(&mut conn, id1)
            .expect("Should exist.")
            .as_state()
            .expect("Should be a state.");
        assert!(current_round_index == bds.current_open_round.expect("Should not be none."));

        //bds.determine_designations_from_current_selections(); //This is called internally by the set_selections_for_current_fround function.

        println!("----------------------");
        println!("Block Division State Serialization:");
        bds.pretty_print();
        println!("----------------------");
        println!("");

        let pertinent_designations = &bds
            .bucket_states
            .get(current_bucket_index)
            .expect("Should exist.")
            .get_state(&current_round_index)
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
