#[tokio::main]
async fn main() {
    println!("Starting Block Divider");
    tokio::spawn(block_divider::tokio_serve());
}
