use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Loading environment variables.");
    dotenvy::dotenv()?;

    println!("Starting Block Divider");
    let server_handle = tokio::spawn(block_divider::tokio_serve());
    match server_handle.await {
        Ok(_) => {
            println!("Server shut down gracefully.");
        }
        Err(e) => {
            eprintln!("Server error: {:?}", e)
        }
    }
    println!("Application finished.");

    Ok(())
}
