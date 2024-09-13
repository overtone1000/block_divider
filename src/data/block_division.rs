use rand::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    error::Error,
    f32::consts::E,
    hash::{Hash, Hasher},
    io::{BufReader, BufWriter},
};

use serde::{Deserialize, Serialize};

use super::{
    bucket::{self, Bucket, BucketDef, BucketState, Ranks},
    participant::Participant,
    selections::Selections,
};

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    round: u64,
    selections: HashMap<Participant, Selections>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockDivisionState {
    participants: Vec<Participant>,
    selection_rounds: u64,
    buckets: HashMap<String, Bucket>,
}

pub type RankFileContents = HashMap<String, Ranks>;

const RANK_CACHE_PATH: &str = "./rank_cache/";

impl BlockDivisionState {
    pub fn build(
        bucket_definitions: HashMap<String, BucketDef>,
        participants: Vec<Participant>,
        selection_rounds: u64,
    ) -> Result<BlockDivisionState, Box<dyn Error>> {
        //Compute the filename
        let mut hasher = std::hash::DefaultHasher::new();
        for bucket in &bucket_definitions {
            bucket.hash(&mut hasher);
        }
        for participant in &participants {
            participant.hash(&mut hasher);
        }
        selection_rounds.hash(&mut hasher);
        let hash = hasher.finish();
        let filename = RANK_CACHE_PATH.to_string() + &hash.to_string();

        let mut retval = BlockDivisionState {
            participants: participants,
            selection_rounds: selection_rounds,
            buckets: HashMap::new(),
        };
        for (bucket_name, bucket_definition) in bucket_definitions {
            retval.buckets.insert(
                bucket_name,
                Bucket {
                    definition: bucket_definition,
                    state: BucketState::default(),
                },
            );
        }

        if std::path::Path::is_file(std::path::Path::new(&filename)) {
            println!("Loading cached ranks.");
            match std::fs::File::open(&filename) {
                Ok(file) => {
                    let bucket_ranks: RankFileContents =
                        serde_json::from_reader(BufReader::new(file))
                            .expect("Couldn't deserialize file.");
                    for (bucket_name, ranks) in bucket_ranks {
                        match retval.buckets.get_mut(&bucket_name) {
                            Some(bucket) => {
                                bucket.state.ranks = ranks;
                            }
                            None => {
                                panic!("Malformed ranks!");
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!("Coudln't open cache file! {:?}", e)
                }
            };
        } else {
            println!("Generating ranks.");
            retval.generate_ranks();
            let mut ranks: RankFileContents = HashMap::new();
            for (bucket_name, bucket) in &retval.buckets {
                ranks.insert(bucket_name.to_string(), bucket.state.ranks.clone());
            }
            std::fs::create_dir_all(RANK_CACHE_PATH).expect("Should be able to make path.");
            match std::fs::File::create_new(&filename) {
                Ok(file) => {
                    serde_json::to_writer(BufWriter::new(file), &ranks)
                        .expect("Error writing hash file!");
                }
                Err(e) => {
                    panic!("Couldn't create hash file! {:?}", e);
                }
            }
        }

        Ok(retval)
    }

    fn generate_ranks(&mut self) {
        let participant_count = self.participants.len() as u64;
        let mut initial_available_ranks: HashSet<u64> = HashSet::new();

        for n in 0..participant_count {
            initial_available_ranks.insert(n);
        }

        let mut rng: ThreadRng = thread_rng();

        println!("Iterating through {} rounds.", self.selection_rounds);
        for round in 0..self.selection_rounds {
            println!("Round {}", round);
            for (bucket_name, bucket) in &mut self.buckets {
                let mut bucket_state_this_round: HashMap<Participant, u64> = HashMap::new();

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
                    let mut available_ranks: HashSet<u64> = HashSet::new();
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

                bucket.state.ranks.insert(round, bucket_state_this_round);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn block_division_state_building() {
        let mut buckets: HashMap<String, BucketDef> = HashMap::new();

        for n in 0..4 {
            buckets.insert(
                "Bucket ".to_string() + &n.to_string(),
                BucketDef {
                    available_slots: 5,
                    available_ancillaries: ["Black Butte".to_string()].to_vec(),
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

        let bds = BlockDivisionState::build(buckets, participants, selection_rounds)
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
    }
}
