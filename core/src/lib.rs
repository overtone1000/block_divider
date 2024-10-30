use std::net::{IpAddr, Ipv4Addr};

use db::database_url;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use diesel_migrations::MigrationHarness;
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

pub const MIGRATIONS: diesel_migrations::EmbeddedMigrations =
    diesel_migrations::embed_migrations!("./migrations");

pub async fn tokio_serve<'a>() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Starting database transaction handler");
    //let mut db_handler: AsyncDatabaseTransactionHandler<DatabaseTransaction, PgConnection> =
    //    AsyncDatabaseTransactionHandler::new(establish_connection);

    let cm = ConnectionManager::<PgConnection>::new(database_url());

    let db_handler = Pool::builder()
        .test_on_check_out(true)
        .build(cm)
        .expect("Could not build connection pool");

    println!("Running pending database migrations");
    match db_handler.get() {
        Ok(mut conn) => match conn.run_pending_migrations(MIGRATIONS) {
            Ok(_) => (),
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Couldn't run migrations. {:?}", e),
                )));
            }
        },
        Err(e) => return Err(Box::new(e)),
    };

    println!("Building server");
    let service = PostHandler::new(db_handler);

    loop {
        println!("Starting server.");

        let server = spawn_server(
            IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            PORT,
            StatefulService::<PostHandler>::create(service.clone()),
        );

        println!("Server up.");

        match tokio::try_join!(server) {
            Ok(_) => {
                println!("Server exited gracefully.");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Server error: {:?}", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}
