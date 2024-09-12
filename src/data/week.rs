use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::participant::Participant;

#[derive(Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Week {
    starting_sunday: NaiveDate,
    designations: Vec<Vec<Participant>>, //Vector containing predesignations in index 0 and each selection round after that
}
