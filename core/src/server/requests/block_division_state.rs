use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct GetStateRequest {
    division_id: String,
}

impl GetStateRequest {
    pub fn get_division_id(&self) -> &str {
        &self.division_id
    }
}

#[cfg(test)]
mod tests {
    use crate::server::requests::BlockDivisionPost;

    use super::GetStateRequest;

    #[test]
    fn serialization() {
        let post = BlockDivisionPost::GetState(GetStateRequest {
            division_id: "Test_Division_ID".to_string(),
        });

        let serialized = serde_json::to_string_pretty(&post).expect("Should serialize.");
        let deserialized =
            serde_json::from_str::<BlockDivisionPost>(&serialized).expect("Should deserialize.");

        println!("{}", serialized);
        assert_eq!(post, deserialized);
    }
}
