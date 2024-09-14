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

use super::{
    bucket::{self, Bucket, BucketDef, BucketName, BucketState, Ranks},
    participant::Participant,
    round::{RoundIndex, RoundName},
    selections::Selection,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BlockDivisionState {
    participant_round_picks: BTreeMap<Participant, BTreeMap<RoundIndex, u64>>,
    selection_rounds: Vec<RoundName>,
    buckets: BTreeMap<BucketName, Bucket>,
    selections: BTreeMap<u64, BTreeMap<Participant, BTreeSet<Selection>>>,
    current_open_round: u64,
}

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    round: u64,
    participant: Participant,
    selection: Selection,
}

const STATE_CACHE_PATH: &str = "./state_cache/";

impl BlockDivisionState {
    fn round_count(&self) -> u64 {
        self.selection_rounds.len() as u64
    }

    fn get_hash(&self) -> u64 {
        let mut hasher = std::hash::DefaultHasher::new();
        for (bucket_name, bucket) in &self.buckets {
            bucket_name.hash(&mut hasher);
            bucket.definition.hash(&mut hasher);
        }
        self.participant_round_picks.hash(&mut hasher);
        self.selection_rounds.hash(&mut hasher);
        hasher.finish()
    }

    fn get_filename(&self) -> String {
        STATE_CACHE_PATH.to_string() + &self.get_hash().to_string()
    }

    pub fn create(
        bucket_definitions: &BTreeMap<BucketName, BucketDef>,
        participant_round_picks: &BTreeMap<Participant, BTreeMap<RoundIndex, u64>>,
        selection_rounds: &Vec<RoundName>,
        use_cache_if_available: bool,
    ) -> Result<BlockDivisionState, Box<dyn Error>> {
        //Initialize the return value enough to calculate the filename.
        let mut retval = {
            let mut initial_state = BlockDivisionState {
                participant_round_picks: participant_round_picks.clone(),
                selection_rounds: selection_rounds.clone(),
                buckets: BTreeMap::new(),
                selections: BTreeMap::new(),
                current_open_round: 0,
            };
            for (bucket_name, bucket_def) in bucket_definitions {
                initial_state.buckets.insert(
                    bucket_name.to_string(),
                    Bucket {
                        definition: bucket_def.clone(),
                        state: BTreeMap::new(),
                    },
                );
            }
            initial_state
        };

        let filename = retval.get_filename();

        let file_exists = std::path::Path::is_file(std::path::Path::new(&filename));

        if use_cache_if_available && file_exists {
            println!("Loading cached ranks.");
            match std::fs::File::open(&filename) {
                Ok(file) => {
                    retval.buckets = serde_json::from_reader(BufReader::new(file))
                        .expect("Couldn't deserialize file.");
                }
                Err(e) => {
                    panic!("Coudln't open cache file! {:?}", e)
                }
            };
        } else {
            //Input bucket definitions
            for (bucket_name, bucket_definition) in bucket_definitions {
                let mut new_bucket = Bucket {
                    definition: bucket_definition.clone(),
                    state: BTreeMap::new(),
                };
                for n in 0..retval.round_count() {
                    new_bucket.state.insert(n, BucketState::default());
                }
                retval.buckets.insert(bucket_name.to_string(), new_bucket);
            }

            println!("Generating ranks.");
            retval.generate_ranks();

            std::fs::create_dir_all(STATE_CACHE_PATH).expect("Should be able to make path.");
            match std::fs::File::create(&filename) {
                Ok(file) => {
                    file.set_len(0).expect("Couldn't clear file.");
                    serde_json::to_writer(BufWriter::new(file), &retval.buckets)
                        .expect("Error writing hash file!");
                }
                Err(e) => {
                    panic!("Couldn't create hash file! {:?}", e);
                }
            }
        }

        Ok(retval)
    }

    pub fn input_selection(
        &mut self,
        participant: Participant,
        selections: BTreeSet<Selection>,
        round: u64,
    ) {
        let pick_count = self
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
    }

    fn determine_designations_from_current_selections(&mut self) {
        for round in 0..self.round_count() {
            match self.selections.get(&round) {
                Some(participant_selections_map) => {
                    for (participant, selections) in participant_selections_map {
                        for selection in selections {
                            let bucket =
                                self.buckets
                                    .get_mut(&selection.bucket_name)
                                    .expect(&format!(
                                        "Key {} should exist in bucket but wasn't found.",
                                        &selection.bucket_name
                                    ));
                            bucket.attempt_selection(&round, participant, selection);
                        }
                    }
                }
                None => {}
            }
        }
    }

    fn save_state(&mut self) {}

    fn generate_ranks(&mut self) {
        let participant_count = self.participant_round_picks.len() as u64;
        let mut initial_available_ranks: BTreeSet<u64> = BTreeSet::new();

        for n in 0..participant_count {
            initial_available_ranks.insert(n);
        }

        let mut rng: ThreadRng = thread_rng();

        println!("Iterating through {} rounds.", self.round_count());
        for round in 0..self.round_count() {
            println!("Round {}", round);
            for (bucket_name, bucket) in &mut self.buckets {
                let mut bucket_state_this_round: BTreeMap<Participant, u64> = BTreeMap::new();

                for participant in self.participant_round_picks.keys() {
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

                bucket.state.get_mut(&round).expect("Should exist.").ranks =
                    bucket_state_this_round;
            }
        }
    }

    pub(crate) fn pretty_print(&self) {
        let serialized = serde_json::to_string_pretty(self).expect("Should serialize.");
        println!("{}", serialized);
    }
}

#[cfg(test)]
mod tests {
    use std::char::ParseCharError;

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

    fn create_basis() -> (
        std::collections::BTreeMap<std::string::String, bucket::BucketDef>,
        std::collections::BTreeMap<std::string::String, std::collections::BTreeMap<u64, u64>>,
        Vec<std::string::String>,
    ) {
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

        (buckets, participants, rounds)
    }

    #[test]
    fn block_division_cache_and_serialization_testing() {
        let (buckets, participants, rounds) = create_basis();

        BlockDivisionState::create(&buckets, &participants, &rounds, false) //create to test overwriting
            .expect("Should work.");
        let bds = BlockDivisionState::create(&buckets, &participants, &rounds, false) //recreate to test ignoring
            .expect("Should work.");

        println!("----------------------");
        println!("Block Division State Serialization:");
        bds.pretty_print();
        println!("----------------------");
        println!("");

        let bds2 = BlockDivisionState::create(&buckets, &participants, &rounds, true) //recreate to test equivalence
            .expect("Should work.");

        assert!(bds == bds2); //bds created from cache must be equal to the one that created the cache
        assert!(
            serde_json::to_string(&bds).expect("Should serialize.")
                == serde_json::to_string(&bds2).expect("Should serialize.")
        ); //serializations should also be equal.
    }

    #[test]
    fn selection_and_calculation() {
        let (buckets, participants, rounds) = create_basis();
        let mut bds = BlockDivisionState::create(&buckets, &participants, &rounds, false)
            .expect("Should work.");

        let currentbucketname = bucketname(1);
        let currentround = 0;

        let mut selections_a: BTreeSet<Selection> = BTreeSet::new();
        let mut ancillaries_a: BTreeSet<String> = BTreeSet::new();
        ancillaries_a.insert(BLACK_BUTTE.to_string());
        selections_a.insert(Selection {
            bucket_name: currentbucketname.to_string(),
            ancillaries: ancillaries_a,
        });
        bds.input_selection(PARTICIPANT_A.to_string(), selections_a, currentround);
        bds.determine_designations_from_current_selections();

        /*
        println!("----------------------");
        println!("Block Division State Serialization:");
        bds.pretty_print();
        println!("----------------------");
        println!("");
        */

        let pertinent_designations = &bds
            .buckets
            .get(&currentbucketname)
            .expect("Should exist.")
            .state
            .get(&currentround)
            .expect("Should exist.")
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
