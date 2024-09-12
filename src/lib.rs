use std::net::{IpAddr, Ipv4Addr};

use server::{commons::spawn_server, handler::PostHandler};

pub mod data;
pub mod server;

const PORT: u16 = 80;

pub async fn tokio_serve<'a>() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Building server");

    let handler = PostHandler::new();
    let server = spawn_server(IpAddr::V4(Ipv4Addr::LOCALHOST), PORT, &handler);

    tokio::try_join!(server)?;

    Ok(())
}
