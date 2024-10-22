use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct DeleteStateRequest {
    id: String,
}

impl DeleteStateRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}
