use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) enum BlockDivisionPost {
    GetState(GetStateRequest),
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct GetStateRequest {
    user_id: String,
    division_id: String,
}

impl GetStateRequest {
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn get_division_id(&self) -> &str {
        &self.division_id
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockDivisionPost, GetStateRequest};

    #[test]
    fn serialization() {
        let post = BlockDivisionPost::GetState(GetStateRequest {
            user_id: "Test_User_ID".to_string(),
            division_id: "Test_Division_ID".to_string(),
        });

        let serialized = serde_json::to_string_pretty(&post).expect("Should serialize.");
        let deserialized =
            serde_json::from_str::<BlockDivisionPost>(&serialized).expect("Should deserialize.");

        println!("{}", serialized);
        assert_eq!(post, deserialized);
    }
}
