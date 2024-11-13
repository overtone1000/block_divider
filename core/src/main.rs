use std::error::Error;

const INSECURE_MODE_ARG: &str = "--insecure";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Checking arguments.");
    let args: Vec<String> = std::env::args().collect();
    let insecure_mode = args.contains(&INSECURE_MODE_ARG.to_string());
    let enable_auth = !insecure_mode;

    println!("Checking for local environment variables.");
    let _ = dotenvy::dotenv().is_err_and(|_| {
        println!("No env file found.");
        true
    });

    loop {
        println!("Starting Block Divider");
        let server_handle = tokio::spawn(block_divider::tokio_serve(enable_auth));
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
