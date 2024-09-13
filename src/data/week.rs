use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::participant::Participant;

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Week {
    starting_sunday: NaiveDate,
}

#[derive(Deserialize, Serialize)]
pub struct WeekState {
    pub(crate) available_slots: u64,
    pub(crate) available_ancillaries: Vec<String>,
    pub(crate) designations: HashMap<u64, Vec<Participant>>, //Map containing predesignations in index 0 and each selection round index after that
    pub(crate) ranks: HashMap<Participant, u64>,
}
