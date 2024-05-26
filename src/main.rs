use std::net::TcpListener;

use zero2prodLibrary::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Unable to bound the address");
    run(listener)?.await
}
