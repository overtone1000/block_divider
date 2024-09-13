use serde::{Deserialize, Serialize};

use super::bucket::BucketDef;

#[derive(Deserialize, Serialize)]
pub struct Selection {
    week: BucketDef,
    ancillaries: Vec<String>, //this is where Black Butte will go but opens it to other possibilities
}

pub type Selections = Vec<Selection>;
