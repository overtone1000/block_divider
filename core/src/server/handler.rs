use hyper_util::client::legacy::connect::Connect;
use serde::Serialize;
use std::{borrow::BorrowMut, collections::BTreeMap, future::Future, num::IntErrorKind, pin::Pin, sync::Arc};
use tokio::sync::{mpsc::{self, UnboundedReceiver, UnboundedSender}};

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

use crate::{
    db::{division::PersistentDivision, key_value::KeyValuePair}, division::{bucket, state::BlockDivisionState}, server::{requests::{block_division_user_view::UserView, BlockDivisionPost}, responses::SingleBlockDivisionState}
};

use super::responses::BlockDivisionServerResponse;

#[derive(Clone)]
pub struct PostHandler {
    database_transaction_handler: Pool<ConnectionManager<PgConnection>>,
    block_division_write_mutices: Arc<std::sync::Mutex<BTreeMap<String,Arc<std::sync::Mutex<()>>>>>
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
            block_division_write_mutices: Arc::new(std::sync::Mutex::new(BTreeMap::new())),
        }
    }

    fn get_block_division_lock(&self,id:&str)->Result<Arc<std::sync::Mutex<()>>,Box<dyn std::error::Error>>
    {
        match self.block_division_write_mutices.lock()
        {
            Ok(mut outer_mutex) => {
                match outer_mutex.entry(id.to_string())
                {
                    std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                        Ok(vacant_entry.insert(Arc::new(std::sync::Mutex::new(()))).clone())
                    },
                    std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                        Ok(occupied_entry.get().clone())
                    },
                }
            },
            Err(_) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other,"Couldn't get outer mutex."))),
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
                    BlockDivisionPost::GetStates(_) => {
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
                    BlockDivisionPost::DeleteState(delete_state_request) => {
                        println!("Delete division.");
                        let res = match PersistentDivision::delete_division(
                            &mut conn,
                            delete_state_request.get_id().to_string(),
                        ) {
                            Ok(_) => true,
                            Err(_) => false,
                        };

                        get_response(Some(res))
                    }
                    BlockDivisionPost::GetUserView(get_user_view_request) => {
                        match UserView::get(&mut conn, get_user_view_request.get_hash()) {
                            Ok(user_view) => {
                                println!("Got request for {:?}", user_view);
                                get_user_view(&mut conn,&user_view)
                            }
                            Err(e) => generic_json_error_from_debug(e),
                        }
                    }

                    BlockDivisionPost::SendStartEmail(send_start_email) => {
                        match mail::get_service_from_env() {
                            Ok(mail_service) => {
                                match PersistentDivision::get_state_from_id(
                                    &mut conn,
                                    send_start_email.get_state_id(),
                                ) {
                                    Ok(state) => match state {
                                        Some(state) => {
                                            match TryInto::<usize>::try_into(
                                                send_start_email.get_user_id(),
                                            ) {
                                                Ok(user_id) => {
                                                    match state
                                                        .basis
                                                        .get_participant_definitions()
                                                        .get(user_id)
                                                    {
                                                        Some(user) => {
                                                            if mail::is_valid_email(
                                                                user.get_email(),
                                                            ) {

                                                                match send_start_email.set(&mut conn)
                                                                {
                                                                    Ok(_)=>{
                                                                        let subject = format!(
                                                                            "Block Division {} Started",
                                                                            send_start_email.get_state_id()
                                                                        );
        
                                                                        let hash =
                                                                            send_start_email.get_hash();
        
                                                                        let body = format!("Enter your selections at http://localhost:5173?hash={}",hash);
        
                                                                        match mail::send_mail(
                                                                            &mail_service,
                                                                            user.get_email(),
                                                                            subject,
                                                                            body,
                                                                        ){
                                                                            Ok(r)=>{
                                                                                if r.is_positive() {
                                                                                    get_response(Some(true))
                                                                                }else {
                                                                                    generic_json_error(&format!("Couldn't send e-mail to {}, error: {}",user.get_email(),r.code()))
                                                                                }
                                                                                },Err(e)=>{generic_json_error_from_debug(e)}
                                                                        }
                                                                    },
                                                                    Err(e)=>generic_json_error_from_debug(e)
                                                                }
                                                                
                                                            } else {
                                                                generic_json_error(&format!(
                                                                    "Invalid email {}",
                                                                    user.get_email()
                                                                ))
                                                            }
                                                        }
                                                        None => generic_json_error("No such user."),
                                                    }
                                                }
                                                Err(e) => generic_json_error("Invalid index."),
                                            }
                                        }
                                        None => generic_json_error("No such state."),
                                    },
                                    Err(e) => generic_json_error_from_debug(e),
                                }
                            }
                            Err(e) => generic_json_error_from_debug(e),
                        }
                    }
                    BlockDivisionPost::SubmitSelections(submit_selections) => {
                        match self.get_block_division_lock(&submit_selections.state_id)
                        {
                            Ok(lock) => {
                                match lock.lock(){
                                    Ok(_)=>{
                                        match BlockDivisionState::set_selections_for_current_round(&mut conn, 
                                            submit_selections.state_id.to_string(),  
                                            submit_selections.user_id, 
                                            submit_selections.selections){
                                                Ok(_) => {
                                                    get_user_view(&mut conn,&UserView::create(submit_selections.user_id as i32,submit_selections.state_id))
                                                },
                                                Err(e) => generic_json_error_from_debug(e),
                                            }
                                    }
                                    Err(_) => generic_json_error("Couldn't lock block id."),
                                }
                            },
                            Err(e) => generic_json_error_from_debug(e),
                        }
                    },
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
    T: BlockDivisionServerResponse
{
    match message {
        Some(result) => Response::new(full_to_boxed_body(
            serde_json::to_string(&result).expect("Couldn't serialize result"),
        )),
        None => generic_json_error("No such state."),
    }
}

fn get_user_view(conn:&mut PgConnection,user_view:&UserView)->Response<
http_body_util::combinators::BoxBody<
    hyper::body::Bytes,
    Box<dyn std::error::Error + Send + Sync>,
>,
>{
    match PersistentDivision::get_state_from_id(
        conn,
        user_view.get_state_id(),
    ) {
        Ok(state) => match state {
            Some(mut state)=>{
                //Censor the final state
                let start = match state.current_open_round
                {
                    Some(start)=>{
                        start+1
                    },
                    None=>{
                        0
                    }
                };
                let fin =state.basis.get_selection_rounds().len();
                println!("Censoring rounds {} - {}",start,fin-1);
                for bucket_state in &mut state.bucket_states
                {
                    for round in start..fin
                    {
                        bucket_state.get_state_mut(&round).ranks=None;
                    }
                }

                get_response(Some(SingleBlockDivisionState
                    {
                        user_id:user_view.get_user_id(),
                        state_id:user_view.get_state_id().to_string(),
                        state:state
                    }))
            },None=>{
                generic_json_error("No such state")
            }
        },
        Err(e) => generic_json_error_from_debug(e),
    }
}