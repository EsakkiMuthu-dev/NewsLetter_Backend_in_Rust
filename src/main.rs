use std::io::stdout;
use std::net::TcpListener;

use sqlx::PgPool;

use zero2prodLibrary::configuration::get_configuration;
use zero2prodLibrary::startup::run;
use zero2prodLibrary::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let tracing_subscriber = get_subscriber("zero2prodLibrary".into(), "info".into(), stdout);
    init_subscriber(tracing_subscriber);

    //panic if we cant read configuration
    let configuration = get_configuration().expect("Failed to read Configuration");
    let connection_pool = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Cant get DB connection");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(address).expect("Unable to bound the address");
    run(listener, connection_pool)?.await
}
