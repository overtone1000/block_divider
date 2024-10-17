use std::net::{IpAddr, Ipv4Addr};

use ::db::AsyncDatabaseTransactionHandler;
use db::{establish_connection, handler::DatabaseTransaction};
use diesel::PgConnection;
use hyper_services::{
    service::{stateful_service::StatefulService, stateless_service::StatelessService},
    spawn_server,
};
use server::handler::PostHandler;

//For Diesel
pub mod schema;

pub mod db;
pub mod division;
pub mod server;

const PORT: u16 = 8181;

pub async fn tokio_serve<'a>() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting database transaction handler");
    let mut db_handler: AsyncDatabaseTransactionHandler<DatabaseTransaction, PgConnection> =
        AsyncDatabaseTransactionHandler::new(establish_connection());

    println!("Building server");
    let service = PostHandler::new(db_handler.get_sender());
    let server = spawn_server(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        PORT,
        StatefulService::<PostHandler>::create(service),
    );

    println!("Server up.");

    tokio::try_join!(server, db_handler.start())?;

    Ok(())
}
