use std::hash::{DefaultHasher, Hash, Hasher};

use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use crate::{
    db::key_value::KeyValuePair,
    division::selections::{Selection, Selections},
};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct SubmitSelections {
    pub(crate) user_id: usize,
    pub(crate) state_id: String,
    pub(crate) selections: Vec<Option<Selection>>,
}

mod tests {
    use std::collections::BTreeSet;

    use crate::division::selections::Selection;

    use super::SubmitSelections;

    //{"SubmitSelections":{"user_id":0,"state_id":"Alpha","selections":[{"bucket_index":0,"ancillaries":{}}]}}
    #[test]
    fn selection_serialization() {
        let s = SubmitSelections {
            user_id: 5,
            state_id: "test".to_string(),
            selections: Vec::from([
                Some(Selection {
                    bucket_index: 5,
                    ancillaries: BTreeSet::from([6, 7]),
                    state: None,
                }),
                None,
                Some(Selection {
                    bucket_index: 8,
                    ancillaries: BTreeSet::from([9, 10, 11, 12]),
                    state: None,
                }),
                None,
            ]),
        };

        let str = serde_json::to_string(&s).expect("Should serialize.");
        println!("Serialzed: {}", &str);

        let res = serde_json::from_str(&str).expect("Should deserialize.");
        assert!(s == res);
    }
}
