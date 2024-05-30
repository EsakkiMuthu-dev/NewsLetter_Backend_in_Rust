use std::net::TcpListener;

use zero2prodLibrary::configuration::get_configuration;
use zero2prodLibrary::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //panic if we cant read configuration
    let configuration = get_configuration().expect("Failed to read Configuration");

    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(address).expect("Unable to bound the address");
    run(listener)?.await
}
