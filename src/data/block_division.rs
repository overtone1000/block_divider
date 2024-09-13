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
    bucket::{self, Bucket, BucketDef, BucketState, Ranks},
    participant::Participant,
    selections::Selection,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub struct BlockDivisionState {
    participants: Vec<Participant>,
    selection_rounds: u64,
    buckets: BTreeMap<String, Bucket>,
    selections: BTreeMap<u64, BTreeMap<Participant, Option<Selection>>>,
}

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    round: u64,
    participant: Participant,
    selection: Selection,
}

const STATE_CACHE_PATH: &str = "./state_cache/";

impl BlockDivisionState {
    pub fn create(
        bucket_definitions: &BTreeMap<String, BucketDef>,
        participants: &Vec<Participant>,
        selection_rounds: u64,
        use_cache_if_available: bool,
    ) -> Result<BlockDivisionState, Box<dyn Error>> {
        //Compute the filename
        let mut hasher = std::hash::DefaultHasher::new();
        for bucket in bucket_definitions {
            bucket.hash(&mut hasher);
        }
        for participant in participants {
            participant.hash(&mut hasher);
        }
        selection_rounds.hash(&mut hasher);
        let hash = hasher.finish();
        let filename = STATE_CACHE_PATH.to_string() + &hash.to_string();

        let mut retval = BlockDivisionState {
            participants: participants.clone(),
            selection_rounds: selection_rounds,
            buckets: BTreeMap::new(),
            selections: BTreeMap::new(),
        };

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
                for n in 0..selection_rounds {
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
        selection: Option<Selection>,
        round: u64,
    ) {
        let round_selections = match self.selections.entry(round) {
            std::collections::btree_map::Entry::Vacant(entry) => entry.insert(BTreeMap::new()),
            std::collections::btree_map::Entry::Occupied(entry) => entry.into_mut(),
        };
        round_selections.insert(participant, selection);
        self.calculate();
    }

    fn calculate(&mut self) {
        todo!("Need to do this!");
    }

    fn generate_ranks(&mut self) {
        let participant_count = self.participants.len() as u64;
        let mut initial_available_ranks: BTreeSet<u64> = BTreeSet::new();

        for n in 0..participant_count {
            initial_available_ranks.insert(n);
        }

        let mut rng: ThreadRng = thread_rng();

        println!("Iterating through {} rounds.", self.selection_rounds);
        for round in 0..self.selection_rounds {
            println!("Round {}", round);
            for (bucket_name, bucket) in &mut self.buckets {
                let mut bucket_state_this_round: BTreeMap<Participant, u64> = BTreeMap::new();

                for participant in &self.participants {
                    println!("");
                    println!(
                        "Participant {} round {} bucket {}",
                        participant, round, bucket_name
                    );

                    println!(
                        "Ranks already used for this bucket and round are {:?}",
                        (&bucket_state_this_round).values()
                    );
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
                    println!(
                        "This leaves the following available ranks for the randomizer {:?}",
                        (&available_ranks_as_vec)
                    );

                    let rank_index = rng.gen_range(0..available_ranks_as_vec.len());
                    let rank = available_ranks_as_vec
                        .get(rank_index)
                        .expect("Should exist.");

                    bucket_state_this_round.insert(participant.to_string(), rank.to_owned());

                    println!(
                        "Adding rank {} for participant {} for round {} in bucket {}",
                        rank, participant, round, bucket_name
                    );
                    println!("");
                }

                bucket.state.get_mut(&round).expect("Should exist.").ranks =
                    bucket_state_this_round;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_division_cache_and_serialization_testing() {
        let mut buckets: BTreeMap<String, BucketDef> = BTreeMap::new();

        for n in 0..4 {
            buckets.insert(
                "Bucket ".to_string() + &n.to_string(),
                BucketDef {
                    available_slots: 5,
                    available_ancillaries: BTreeSet::from(["Black Butte".to_string()]),
                },
            );
        }

        let participants = [
            "Participant A".to_string(),
            "Participant B".to_string(),
            "Participant C".to_string(),
        ]
        .to_vec();

        let selection_rounds = 3;

        BlockDivisionState::create(&buckets, &participants, selection_rounds, false) //create to test overwriting
            .expect("Should work.");
        let bds = BlockDivisionState::create(&buckets, &participants, selection_rounds, false) //recreate to test ignoring
            .expect("Should work.");

        println!("----------------------");
        println!("Block Division State:");
        println!("{:?}", bds);
        println!("----------------------");
        println!("");

        let serialized = serde_json::to_string_pretty(&bds).expect("Should serialize.");
        println!("----------------------");
        println!("Block Division State Serialization:");
        println!("{}", serialized);
        println!("----------------------");
        println!("");

        let bds2 = BlockDivisionState::create(&buckets, &participants, selection_rounds, true) //recreate to test equivalence
            .expect("Should work.");

        assert!(bds == bds2); //bds created from cache must be equal to the one that created the cache
        assert!(
            serde_json::to_string(&bds).expect("Should serialize.")
                == serde_json::to_string(&bds2).expect("Should serialize.")
        ); //serializations should also be equal.
    }
}
