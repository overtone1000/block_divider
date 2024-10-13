use std::{future::Future, pin::Pin, sync::Arc};

use diesel::PgConnection;
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{Bytes, Frame, Incoming},
    service::Service,
    Method, Request, Response,
};
use hyper_services::{
    commons::{HandlerBody, HandlerError, HandlerFuture, HandlerResponse, HandlerResult},
    cors::permit_all_cors,
    request_processing::get_request_body_as_string,
    response_building::{full_to_boxed_body, not_found, send_file},
    service::stateless_service::StatelessHandler,
};

use crate::db::{establish_connection, ConnectionHandler};

use super::block_division_state_request::BlockDivisionPost;

#[derive(Clone)]
pub struct PostHandler {}

const BLOCK_DIVISION_POST: &str = "/block_division_post";
const ECHO: &str = "/echo";

impl StatelessHandler for PostHandler {
    async fn handle_request(request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path();
        //let headers = request.headers().clone();

        println!("Method: {}, Path: {}", method, path);

        match (method, path) {
            (Method::POST, BLOCK_DIVISION_POST) => Self::handle_post(request).await,
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

    async fn handle_post(request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;

        let mut response = match serde_json::from_str::<BlockDivisionPost>(&as_string) {
            Ok(request_body) => {
                println!("Received request");
                Response::new(full_to_boxed_body("Not yet implemented"))
            }
            Err(_e) => {
                eprintln!("Could not parse package. {:?}", &as_string);
                Response::new(full_to_boxed_body("Invalid JSON"))
            }
        };

        permit_all_cors(&mut response);

        return Ok(response);
    }
}
