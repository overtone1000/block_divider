use hyper_util::client::legacy::connect::Connect;
use serde::Serialize;
use std::{borrow::BorrowMut, future::Future, pin::Pin, sync::Arc};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    IntoSql, PgConnection,
};
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

use crate::{db::division::PersistentDivision, server::requests::BlockDivisionPost};

#[derive(Clone)]
pub struct PostHandler {
    database_transaction_handler: Pool<ConnectionManager<PgConnection>>,
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
    pub fn new(database_transaction_handler: Pool<ConnectionManager<PgConnection>>) -> PostHandler {
        PostHandler {
            database_transaction_handler: database_transaction_handler,
        }
    }

    fn get_conn(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Box<dyn std::error::Error>> {
        match self.database_transaction_handler.get() {
            Ok(conn) => Ok(conn),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn handle_post(&mut self, request: Request<Incoming>) -> HandlerResult {
        let as_string = get_request_body_as_string(request).await?;

        println!("Handling request {}", &as_string);

        let mut conn = match self.get_conn() {
            Ok(conn) => conn,
            Err(err) => {
                eprintln!("{}", err);
                return Ok(permit_all_cors(generic_json_error_from_debug(err)));
            }
        };

        let mut response = {
            match serde_json::from_str::<BlockDivisionPost>(&as_string) {
                Ok(request_body) => match request_body {
                    BlockDivisionPost::GetState(get_state_request) => {
                        let res = PersistentDivision::get_state_from_id(
                            &mut conn,
                            get_state_request.get_id(),
                        );

                        let response = match res {
                            Ok(res) => res,
                            Err(e) => {
                                eprintln!("{}", e);
                                None
                            }
                        };

                        get_response(response)
                    }
                    BlockDivisionPost::GetDivisions(_) => {
                        let res = PersistentDivision::get_all(&mut conn)
                            .expect("Couldn't get all from persistent division table.");
                        get_response(Some(res))
                    }
                    BlockDivisionPost::SetState(set_state_request) => {
                        let res = match PersistentDivision::update(
                            &mut conn,
                            set_state_request.get_id().to_string(),
                            set_state_request.get_state(),
                        ) {
                            Ok(_) => true,
                            Err(_) => false,
                        };
                        get_response(Some(res))
                    }
                    BlockDivisionPost::NewBasis(new_basis_request) => {
                        println!("New persistent division.");
                        let res = match PersistentDivision::new(
                            &mut conn,
                            new_basis_request.get_id().to_string(),
                            new_basis_request.get_basis(),
                        ) {
                            Ok(_) => true,
                            Err(_) => false,
                        };

                        get_response(Some(res))
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

fn get_response<T>(
    message: Option<T>,
) -> Response<
    http_body_util::combinators::BoxBody<
        hyper::body::Bytes,
        Box<dyn std::error::Error + Send + Sync>,
    >,
>
where
    T: Serialize,
{
    match message {
        Some(result) => Response::new(full_to_boxed_body(
            serde_json::to_string(&result).expect("Couldn't serialize result"),
        )),
        None => generic_json_error("No such state."),
    }
}
