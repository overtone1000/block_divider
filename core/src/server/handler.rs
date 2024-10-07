use std::{future::Future, pin::Pin};

use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    service::Service,
    Method, Request, Response,
};
use hyper_services::{
    commons::{
        full_to_boxed_body, get_request_body_as_string, not_found, send_file, HandlerBody,
        HandlerError, HandlerFuture, HandlerResponse, HandlerResult,
    },
    service::stateless_service::StatelessHandler,
};

use crate::division::block_division::BlockDivisionInput;

use futures_util::TryStreamExt;

use super::block_division_post::BlockDivisionStateRequestBody;

#[derive(Clone)]
pub struct PostHandler {}

const BLOCK_DIVISION_STATE: &str = "/block_division_state";
const BLOCK_DIVISION_INPUT: &str = "/block_division_input";
const ECHO: &str = "/echo";

impl StatelessHandler for PostHandler {
    async fn handle_request(request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path();
        let headers = request.headers().clone();

        println!("Method: {}, Path: {}", method, path);

        match (method, path) {
            (Method::POST, BLOCK_DIVISION_STATE) => Self::bdd(request).await,
            (Method::POST, ECHO) => Self::echo(request).await,
            (Method::GET, path) => send_file(path.to_string()).await,
            _ => {
                return Ok(not_found());
            }
        }
    }
}

impl PostHandler {
    pub fn new() -> PostHandler {
        PostHandler {}
    }

    async fn echo(request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;
        Ok(Response::new(full_to_boxed_body(as_string)))
    }

    async fn bdd(request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;

        let mut response = match serde_json::from_str::<BlockDivisionStateRequestBody>(&as_string) {
            Ok(request_body) => {
                println!("Received request");
                Response::new(full_to_boxed_body("Not yet implemented"))
            }
            Err(_e) => {
                eprintln!("Could not parse package. {:?}", &as_string);
                Response::new(full_to_boxed_body("Invalid JSON"))
            }
        };

        hyper_services::commons::permit_all_cors(&mut response);

        return Ok(response);
    }
}
