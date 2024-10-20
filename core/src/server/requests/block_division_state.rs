use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct GetStateRequest {
    id: String,
}

impl GetStateRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use crate::server::requests::BlockDivisionPost;

    use super::GetStateRequest;

    #[test]
    fn serialization() {
        let post = BlockDivisionPost::GetState(GetStateRequest {
            id: "Test Serialization".to_string(),
        });

        let serialized = serde_json::to_string_pretty(&post).expect("Should serialize.");
        let deserialized =
            serde_json::from_str::<BlockDivisionPost>(&serialized).expect("Should deserialize.");

        println!("{}", serialized);
        assert_eq!(post, deserialized);
    }
}
