use std::collections::BTreeMap;

use chrono::round;
use serde::{Deserialize, Serialize};

use super::round::RoundIndex;

pub type ParticipantIndex = usize;

#[derive(Clone, Deserialize, Serialize, Hash, PartialEq, Eq, Debug)]
pub struct ParticipantDef {
    name: String,
    email: String,
    round_picks_allowed: BTreeMap<RoundIndex, u64>,
}

impl ParticipantDef {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_round_picks_allowed(&self) -> &BTreeMap<RoundIndex, u64> {
        &self.round_picks_allowed
    }

    pub fn create(
        name: String,
        email: String,
        round_picks_allowed: BTreeMap<RoundIndex, u64>,
    ) -> ParticipantDef {
        ParticipantDef {
            name: name,
            email: email,
            round_picks_allowed: round_picks_allowed,
        }
    }
}
