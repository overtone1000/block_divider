use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Checking for local environment variables.");

    let _ = dotenvy::dotenv().is_err_and(|_| {
        println!("No env file found.");
        true
    });

    loop {
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
    }
}
