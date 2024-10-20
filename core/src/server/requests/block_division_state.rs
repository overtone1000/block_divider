use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct GetStateRequest {
    id: i32,
}

impl GetStateRequest {
    pub fn get_id(&self) -> i32 {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use crate::server::requests::BlockDivisionPost;

    use super::GetStateRequest;

    #[test]
    fn serialization() {
        let post = BlockDivisionPost::GetState(GetStateRequest { id: 900 });

        let serialized = serde_json::to_string_pretty(&post).expect("Should serialize.");
        let deserialized =
            serde_json::from_str::<BlockDivisionPost>(&serialized).expect("Should deserialize.");

        println!("{}", serialized);
        assert_eq!(post, deserialized);
    }
}
