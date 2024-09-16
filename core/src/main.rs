#[tokio::main]
async fn main() {
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
}
