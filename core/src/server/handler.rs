use std::{future::Future, pin::Pin};

use http_body_util::{BodyExt, Full};
use hyper::{
    body::{Bytes, Incoming},
    service::Service,
    Request, Response,
};
use serde::{Deserialize, Serialize};

use crate::data::block_division::BlockDivisionInput;

#[derive(Clone)]
pub struct PostHandler {}

impl Service<Request<Incoming>> for PostHandler {
    type Response = Response<Full<Bytes>>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        println!("Handling event.");
        let result = Self::handle_poll(self.clone(), request);
        Box::pin(result)
    }
}

impl PostHandler {
    pub fn new() -> PostHandler {
        PostHandler {}
    }

    async fn handle_poll(
        self: PostHandler,
        request: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>, Box<dyn std::error::Error + Send + Sync>> {
        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let headers = request.headers().clone();

        let as_string = String::from_utf8(request.collect().await?.to_bytes().to_vec())
            .expect("Couldn't parse bytes.");

        let bdd: BlockDivisionInput = match serde_json::from_str(&as_string) {
            Ok(data) => data,
            Err(_e) => {
                eprintln!("Could not parse package. {:?}", &as_string);
                return Ok(Response::new(Full::new(Bytes::from("Invalid JSON"))));
            }
        };

        return Ok(Response::new(Full::new(Bytes::from("Ok"))));
    }
}
