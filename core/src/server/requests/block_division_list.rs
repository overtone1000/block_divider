use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub(crate) struct GetListRequest {}

#[cfg(test)]
mod tests {
    use crate::server::requests::BlockDivisionPost;

    use super::GetListRequest;

    #[test]
    fn serialization() {
        let post = BlockDivisionPost::GetDivisions(GetListRequest {});

        let serialized = serde_json::to_string_pretty(&post).expect("Should serialize.");
        let deserialized =
            serde_json::from_str::<BlockDivisionPost>(&serialized).expect("Should deserialize.");

        println!("{}", serialized);
        assert_eq!(post, deserialized);
    }
}
