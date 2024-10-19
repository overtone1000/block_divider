use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use serde::{Deserialize, Serialize};

use super::{
    bucket::{BucketDef, BucketIndex},
    participant::{ParticipantDef, ParticipantIndex},
    round::{RoundIndex, RoundName},
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub struct BlockDivisionBasis {
    label: String,
    bucket_definitions: Vec<BucketDef>,
    participant_definitions: Vec<ParticipantDef>,
    selection_round_names: Vec<RoundName>,
}

impl BlockDivisionBasis {
    pub fn create(
        label: String,
        bucket_definitions: Vec<BucketDef>,
        participant_definitions: Vec<ParticipantDef>,
        selection_round_names: Vec<RoundName>,
    ) -> BlockDivisionBasis {
        BlockDivisionBasis {
            label: label,
            bucket_definitions: bucket_definitions,
            participant_definitions: participant_definitions,
            selection_round_names: selection_round_names,
        }
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn get_selection_rounds(&self) -> &Vec<RoundName> {
        &self.selection_round_names
    }

    pub fn get_bucket_definitions(&self) -> &Vec<BucketDef> {
        &self.bucket_definitions
    }

    pub fn get_participant_definitions(&self) -> &Vec<ParticipantDef> {
        &self.participant_definitions
    }

    pub fn get_bucket_definition(&self, index: BucketIndex) -> &BucketDef {
        self.bucket_definitions
            .get(index)
            .expect("Bucket not found.")
    }

    pub fn get_participant_definition(&self, index: BucketIndex) -> &ParticipantDef {
        self.participant_definitions
            .get(index)
            .expect("Participant not found.")
    }

    pub fn get_selection_round_name(&self, index: BucketIndex) -> String {
        self.selection_round_names
            .get(index)
            .expect("Selection round not found.")
            .to_string()
    }

    pub fn get_hash(&self) -> String {
        let mut hasher = std::hash::DefaultHasher::new();
        for bucket_definition in &self.bucket_definitions {
            bucket_definition.hash(&mut hasher);
        }
        self.participant_definitions.hash(&mut hasher);
        self.selection_round_names.hash(&mut hasher);
        hasher.finish().to_string()
    }
}
