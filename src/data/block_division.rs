use rand::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    io::{BufReader, BufWriter, Read},
};

use serde::{Deserialize, Serialize};

use super::{
    participant::Participant,
    selections::Selections,
    week::{Week, WeekState},
};

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionInput {
    round: u64,
    selections: HashMap<Participant, Selections>,
}

#[derive(Deserialize, Serialize)]
pub struct BlockDivisionState {
    participants: Vec<Participant>,
    week_states: HashMap<Week, WeekState>, //Weeks being selected
}

impl BlockDivisionState {
    pub fn build(weeks: Vec<Week>, participants: Vec<Participant>) -> BlockDivisionState {
        let mut hasher = std::hash::DefaultHasher::new();
        for week in &weeks {
            week.hash(&mut hasher);
        }
        for participant in &participants {
            participant.hash(&mut hasher);
        }
        let hash = hasher.finish();

        let filename = "./ranks/".to_string() + &hash.to_string();

        let mut retval = BlockDivisionState {
            participants: participants,
            week_states: HashMap::new(),
        };

        match std::fs::File::open(&filename) {
            Ok(file) => {
                let ranks: HashMap<Week, Vec<Participant>> =
                    serde_json::from_reader(BufReader::new(file))
                        .expect("Couldn't deserialize file.");
                for (week, ranks) in ranks {
                    match retval.week_states.get_mut(&week) {
                        Some(week_state) => {
                            week_state.ranks = ranks;
                        }
                        None => {
                            eprintln!("Malformed ranks!");
                        }
                    }
                }
            }
            Err(_) => {
                retval.generate_ranks();
                let mut ranks: HashMap<Week, Vec<Participant>> = HashMap::new();
                for (week, week_state) in &retval.week_states {
                    ranks.insert(week.clone(), week_state.ranks.clone());
                }
                match std::fs::File::create_new(&filename) {
                    Ok(file) => {
                        serde_json::to_writer(BufWriter::new(file), &ranks)
                            .expect("Error writing hash file!");
                    }
                    Err(_) => {
                        eprintln!("Couldn't create hash file!")
                    }
                }
            }
        }

        retval
    }

    fn generate_ranks(&mut self) {
        let mut available_ranks_for_participant_map: HashMap<Participant, Vec<u64>> =
            HashMap::new();
        let participant_count = self.participants.len() as u64;
        let mut initial_available_ranks: Vec<u64> = Vec::new();

        for n in 0..participant_count {
            initial_available_ranks.push(n);
        }

        for participant in &self.participants {
            available_ranks_for_participant_map
                .insert(participant.to_string(), initial_available_ranks.clone());
        }

        let mut rng: ThreadRng = thread_rng();

        for (week, week_state) in &mut self.week_states {
            //Reset any that have run out of possible ranks
            for (_, currently_available_ranks) in &mut available_ranks_for_participant_map {
                if currently_available_ranks.len() <= 0 {
                    *currently_available_ranks = initial_available_ranks.clone();
                }
            }

            for participant in &self.participants {
                let available_ranks_for_this_participant = available_ranks_for_participant_map
                    .get_mut(participant)
                    .expect("Uninitialized available ranks");

                let mut available_ranks: HashSet<u64> = HashSet::new();
                for r in available_ranks_for_this_participant.iter() {
                    available_ranks.insert(r.clone());
                }
                for r in week_state.ranks.values() {
                    available_ranks.remove(r);
                }
                let mut available_ranks_as_vec: Vec<u64> = Vec::new();
                for r in available_ranks {
                    available_ranks_as_vec.push(r.to_owned());
                }

                let rank_index = rng.gen_range(0..available_ranks_as_vec.len());
                let rank = available_ranks_as_vec
                    .get(rank_index)
                    .expect("Should exist.");

                available_ranks_for_this_participant.remove(rank_index);
                week_state
                    .ranks
                    .insert(participant.to_string(), rank.to_owned());
            }
        }
    }
}
