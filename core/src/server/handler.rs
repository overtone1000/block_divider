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
    generic_json_error::{generic_json_error, generic_json_error_from_debug},
    request_processing::get_request_body_as_string,
    response_building::{full_to_boxed_body, not_found, send_file},
    service::stateful_service::StatefulHandler,
};

use crate::{
    db::handler::DatabaseTransaction, server::requests::block_division_state::BlockDivisionPost,
};

#[derive(Clone)]
pub struct PostHandler {
    database_transaction_handler: Sender<DatabaseTransaction>,
}

const BLOCK_DIVISION: &str = "/block_division_post";

impl StatefulHandler for PostHandler {
    async fn handle_request(mut self: Self, request: Request<Incoming>) -> HandlerResult {
        let method = request.method().clone();
        let path = request.uri().path();
        //let headers = request.headers().clone();

        println!("Method: {}, Path: {}", method, path);

        match (method, path) {
            (Method::POST, BLOCK_DIVISION) => {
                println!("Block division post.");
                Self::handle_post(&mut self, request).await
            }
            (Method::GET, path) => {
                println!("File request: {}", path);
                send_file(path.to_string()).await
            }
            _ => {
                eprintln!("Not found.");
                Ok(permit_all_cors(not_found()))
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

        println!("Handling request {}", &as_string);

        let mut response = match serde_json::from_str::<BlockDivisionPost>(&as_string) {
            Ok(request_body) => match request_body {
                BlockDivisionPost::GetState(get_state_request) => {
                    let (sender, receiver) = tokio::sync::oneshot::channel();

                    println!("Awaiting transaction result.");
                    let state = self
                        .database_transaction_handler
                        .send(DatabaseTransaction::GetBlockDivisionState(
                            get_state_request.get_division_id().to_string(),
                            sender,
                        ))
                        .await;

                    match state {
                        Ok(_) => {
                            println!("Awaiting one shot response.");
                            let result = receiver.await;
                            match result {
                                Ok(result) => match result {
                                    Some(result) => Response::new(full_to_boxed_body(
                                        serde_json::to_string(&result)
                                            .expect("Couldn't serialize result"),
                                    )),
                                    None => generic_json_error("No such state."),
                                },
                                Err(err) => generic_json_error_from_debug(err),
                            }
                        }
                        Err(err) => generic_json_error_from_debug(err),
                    }
                }
            },
            Err(err) => generic_json_error_from_debug(err),
        };

        response = permit_all_cors(response);
        println!("Returning {:?}", response);
        return Ok(response);
    }
}
