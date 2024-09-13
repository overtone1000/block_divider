use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::bucket::BucketDef;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
pub struct Selection {
    pub(crate) bucket_name: String,
    pub(crate) ancillaries: BTreeSet<String>, //this is where Black Butte will go but opens it to other possibilities
}
