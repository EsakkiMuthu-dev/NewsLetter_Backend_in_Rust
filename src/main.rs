use std::net::TcpListener;

use env_logger::Env;
use sqlx::PgPool;

use zero2prodLibrary::configuration::get_configuration;
use zero2prodLibrary::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //setting the looger for instumentation
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    //panic if we cant read configuration
    let configuration = get_configuration().expect("Failed to read Configuration");
    let connection_pool = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Cant get DB connection");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(address).expect("Unable to bound the address");
    run(listener, connection_pool)?.await
}
