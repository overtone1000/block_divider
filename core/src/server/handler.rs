use std::{future::Future, pin::Pin};

use http_body_util::{BodyExt, Either, Empty, Full};
use hyper::{
    body::{Body, Bytes, Incoming},
    service::Service,
    Request, Response,
};
use serde::{Deserialize, Serialize};

use crate::data::block_division::BlockDivisionInput;

#[derive(Clone)]
pub struct PostHandler {}

type HandlerResponse = Response<Full<Bytes>>;

impl Service<Request<Incoming>> for PostHandler {
    type Response = HandlerResponse;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        let result = Self::handle_request(request);
        Box::pin(result)
    }
}

impl PostHandler {
    pub fn new() -> PostHandler {
        PostHandler {}
    }

    async fn handle_request(
        request: Request<Incoming>,
    ) -> Result<HandlerResponse, Box<dyn std::error::Error + Send + Sync>> {
        let method = request.method().clone();
        let path = request.uri().path().to_string();
        let headers = request.headers().clone();

        println!("Path: {}", path);

        match path.as_str() {
            "bdd" => Self::bdd(request).await,
            "echo" => Self::echo(request).await,
            _ => {
                return Ok(Self::not_found());
            }
        }
    }

    fn not_found() -> Response<Full<Bytes>> {
        Response::builder()
            .status(hyper::StatusCode::NOT_FOUND)
            .body(Full::new(Bytes::from("Resource not found.")))
            .expect("Should produce response.")
    }

    async fn get_request_body_as_string(
        request: Request<Incoming>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        Ok(
            String::from_utf8(request.collect().await?.to_bytes().to_vec())
                .expect("Couldn't parse bytes."),
        )
    }

    async fn echo(
        request: Request<Incoming>,
    ) -> Result<HandlerResponse, Box<dyn std::error::Error + Send + Sync>> {
        let as_string = Self::get_request_body_as_string(request).await?;
        Ok(Response::new(Full::new(Bytes::from(as_string))))
    }

    async fn bdd(
        request: Request<Incoming>,
    ) -> Result<HandlerResponse, Box<dyn std::error::Error + Send + Sync>> {
        let as_string = Self::get_request_body_as_string(request).await?;

        let bdd: BlockDivisionInput = match serde_json::from_str(&as_string) {
            Ok(data) => data,
            Err(_e) => {
                eprintln!("Could not parse package. {:?}", &as_string);
                return Ok(Response::new(Full::new(Bytes::from("Invalid JSON"))));
            }
        };

        return Ok(Response::new(Full::new(Bytes::from("Not yet implemented"))));
    }
}
