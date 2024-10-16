use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::mpsc::{self, Receiver, Sender};

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
    service::stateful_service::StatefulHandler,
};

use crate::db::{establish_connection, handler::DatabaseTransaction};

use super::block_division_state_request::BlockDivisionPost;

#[derive(Clone)]
pub struct PostHandler {
    database_transaction_handler: Sender<DatabaseTransaction>,
}

const BLOCK_DIVISION_POST: &str = "/block_division_post";
const ECHO: &str = "/echo";

impl StatefulHandler for PostHandler {
    async fn handle_request(mut self: Self, request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path();
        //let headers = request.headers().clone();

        println!("Method: {}, Path: {}", method, path);

        match (method, path) {
            (Method::POST, BLOCK_DIVISION_POST) => Self::handle_post(&mut self, request).await,
            (Method::GET, path) => send_file(path.to_string()).await,
            _ => {
                return Ok(not_found());
            }
        }
    }
}

impl PostHandler {
    pub fn new(database_transaction_handler: Sender<DatabaseTransaction>) -> PostHandler {
        PostHandler {
            database_transaction_handler: database_transaction_handler,
        }
    }

    async fn handle_post(&mut self, request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;

        let mut response = match serde_json::from_str::<BlockDivisionPost>(&as_string) {
            Ok(request_body) => {
                match request_body {
                    BlockDivisionPost::GetState(get_state_request) => {
                        let state = self.database_transaction_handler.send(
                            DatabaseTransaction::PersistentDivision_Get(
                                get_state_request.get_division_id().to_string(),
                            ),
                        );
                    }
                }
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
