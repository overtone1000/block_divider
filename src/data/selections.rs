use serde::{Deserialize, Serialize};

use super::week::Week;

#[derive(Deserialize, Serialize)]
pub struct Selection {
    week: Week,
    ancillaries: Vec<String>, //this is where Black Butte will go but opens it to other possibilities
}

pub type Selections = Vec<Selection>;
