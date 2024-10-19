use serde::Serialize;
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use diesel::{IntoSql, PgConnection};
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

use crate::{db::handler::DatabaseTransaction, server::requests::BlockDivisionPost};

#[derive(Clone)]
pub struct PostHandler {
    database_transaction_handler: UnboundedSender<DatabaseTransaction>,
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
    pub fn new(database_transaction_handler: UnboundedSender<DatabaseTransaction>) -> PostHandler {
        PostHandler {
            database_transaction_handler: database_transaction_handler,
        }
    }

    async fn handle_post(&mut self, request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;

        println!("Handling request {}", &as_string);

        let mut response = {
            match serde_json::from_str::<BlockDivisionPost>(&as_string) {
                Ok(request_body) => match request_body {
                    BlockDivisionPost::GetState(get_state_request) => {
                        let (sender, receiver) = tokio::sync::oneshot::channel();
                        let transaction = DatabaseTransaction::GetBlockDivisionState(
                            get_state_request.get_division_id().to_string(),
                            sender,
                        );
                        database_handler_query(
                            transaction,
                            &mut self.database_transaction_handler,
                            receiver,
                        )
                        .await
                    }
                    BlockDivisionPost::GetDivisions(_) => {
                        let (sender, receiver) = tokio::sync::oneshot::channel();
                        let transaction = DatabaseTransaction::GetBlockDivisionList(sender);
                        database_handler_query(
                            transaction,
                            &mut self.database_transaction_handler,
                            receiver,
                        )
                        .await
                    }
                    BlockDivisionPost::SetState(set_state_request) => {
                        let (sender, receiver) = tokio::sync::oneshot::channel();
                        let transaction =
                            DatabaseTransaction::SetBlockDivisionState(set_state_request, sender);
                        database_handler_query(
                            transaction,
                            &mut self.database_transaction_handler,
                            receiver,
                        )
                        .await
                    }
                },
                Err(err) => generic_json_error_from_debug(err),
            }
        };

        response = permit_all_cors(response);
        println!("Returning {:?}", response);
        return Ok(response);
    }
}

async fn database_handler_query<T>(
    transaction: DatabaseTransaction,
    transaction_handler: &mut UnboundedSender<DatabaseTransaction>,
    receiver: tokio::sync::oneshot::Receiver<Option<T>>,
) -> Response<
    http_body_util::combinators::BoxBody<
        hyper::body::Bytes,
        Box<dyn std::error::Error + Send + Sync>,
    >,
>
where
    T: Serialize,
{
    println!("Awaiting transaction result.");
    let state = transaction_handler.send(transaction);

    match state {
        Ok(_) => get_oneshot_response::<T>(receiver).await,
        Err(err) => generic_json_error_from_debug(err),
    }
}

async fn get_oneshot_response<T>(
    receiver: tokio::sync::oneshot::Receiver<Option<T>>,
) -> Response<
    http_body_util::combinators::BoxBody<
        hyper::body::Bytes,
        Box<dyn std::error::Error + Send + Sync>,
    >,
>
where
    T: Serialize,
{
    println!("Awaiting one shot response.");
    let result = receiver.await;
    match result {
        Ok(result) => match result {
            Some(result) => Response::new(full_to_boxed_body(
                serde_json::to_string(&result).expect("Couldn't serialize result"),
            )),
            None => generic_json_error("No such state."),
        },
        Err(err) => generic_json_error_from_debug(err),
    }
}
