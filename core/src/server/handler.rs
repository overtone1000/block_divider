use std::{future::Future, pin::Pin};

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    service::Service,
    Method, Request, Response,
};
use hyper_trm::{
    commons::{
        full_to_boxed_body, get_request_body_as_string, not_found, send_file, HandlerBody,
        HandlerError, HandlerFuture, HandlerResponse, HandlerResult,
    },
    generic_service::Handler,
};

use crate::data::block_division::BlockDivisionInput;

use futures_util::TryStreamExt;

#[derive(Clone)]
pub struct PostHandler {}

impl Service<Request<Incoming>> for PostHandler {
    type Response = HandlerResponse;
    type Error = HandlerError;
    type Future = HandlerFuture;

    fn call(&self, request: Request<Incoming>) -> Self::Future {
        let result = Self::handle_request(request);
        Box::pin(result)
    }
}

impl PostHandler {
    pub fn new() -> PostHandler {
        PostHandler {}
    }

    async fn handle_request(request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path();
        let headers = request.headers().clone();

        println!("Path: {}", path);

        match (method, path) {
            (Method::POST, "bdd") => Self::bdd(request).await,
            (Method::POST, "echo") => Self::echo(request).await,
            (Method::GET, path) => send_file(path.to_string()).await,
            _ => {
                return Ok(not_found());
            }
        }
    }

    async fn echo(request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;
        Ok(Response::new(full_to_boxed_body(as_string)))
    }

    async fn bdd(request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;

        let bdd: BlockDivisionInput = match serde_json::from_str(&as_string) {
            Ok(data) => data,
            Err(_e) => {
                eprintln!("Could not parse package. {:?}", &as_string);
                return Ok(Response::new(full_to_boxed_body("Invalid JSON")));
            }
        };

        return Ok(Response::new(full_to_boxed_body("Not yet implemented")));
    }
}
