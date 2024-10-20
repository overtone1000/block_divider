use diesel::prelude::*;

use std::net::{IpAddr, Ipv4Addr};

use db::{database_url, establish_connection};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
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
    //let mut db_handler: AsyncDatabaseTransactionHandler<DatabaseTransaction, PgConnection> =
    //    AsyncDatabaseTransactionHandler::new(establish_connection);

    let cm = ConnectionManager::<PgConnection>::new(database_url());

    let db_handler = Pool::builder()
        .test_on_check_out(true)
        .build(cm)
        .expect("Could not build connection pool");

    println!("Building server");
    let service = PostHandler::new(db_handler);
    let server = spawn_server(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        PORT,
        StatefulService::<PostHandler>::create(service),
    );

    println!("Server up.");

    tokio::try_join!(server)?;

    Ok(())
}
