use std::error::Error;

use http_body_util::combinators::BoxBody;
use hyper::Response;
use hyper_services::response_building::full_to_boxed_body;
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
    pub fn get_response(
        &self,
    ) -> Response<BoxBody<hyper::body::Bytes, Box<(dyn Error + Send + Sync + 'static)>>> {
        match self {
            BlockDivisionPost::GetState(get_state_request) => {
                Response::new(full_to_boxed_body("Not yet implemented"))
            }
        }
    }
}
