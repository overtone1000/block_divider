use hyper::Response;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) enum BlockDivisionPost {
    GetState(GetStateRequest),
}

#[derive(Deserialize)]
pub(crate) struct GetStateRequest {
    user_id: String,
    division_id: String,
}

impl BlockDivisionPost {
    pub fn get_response(&self) -> Response<String> {
        match self {
            BlockDivisionPost::GetState(get_state_request) => {}
        }
    }
}
