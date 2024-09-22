use std::net::{IpAddr, Ipv4Addr};

use hyper_services::{service::stateless_service::StatelessService, spawn_server};
use server::handler::PostHandler;

//For Diesel
pub mod schema;

pub mod db;
pub mod division;
pub mod server;

const PORT: u16 = 8181;

pub async fn tokio_serve<'a>() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Building server");

    let server = spawn_server(
        IpAddr::V4(Ipv4Addr::LOCALHOST),
        PORT,
        StatelessService::<PostHandler>::create(),
    );

    tokio::try_join!(server)?;

    Ok(())
}
