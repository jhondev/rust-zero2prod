use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = String::from("127.0.0.1:8000");
    let listener = TcpListener::bind(&address).expect("Failed to bind address");

    println!("Starting server {}", &address);
    run(listener)?.await
}
