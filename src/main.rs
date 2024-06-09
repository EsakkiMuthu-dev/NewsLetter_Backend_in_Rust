use std::net::TcpListener;

use sqlx::PgPool;
use tracing::Dispatch;
use tracing::dispatcher::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, Registry};

use zero2prodLibrary::configuration::get_configuration;
use zero2prodLibrary::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    
    LogTracer::init().expect("Failed to intialise log tracer");

    //env filter to filter based on log levels
    let env_filter =EnvFilter::try_from_default_env().unwrap_or( EnvFilter::new("info"));

    // lets add formatting layer
    let formatting_layer =BunyanFormattingLayer::new(
        "zero2prodLibrary".into(),
        std::io::stdout
    );


    //lets add subscriber for tracing
    let tracing_subscriber =  Registry::default().
        with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);


    // set galobal deafult

    set_global_default(Dispatch::from(tracing_subscriber)).expect("Failed to set tacing Subscriber");

    //panic if we cant read configuration
    let configuration = get_configuration().expect("Failed to read Configuration");
    let connection_pool = PgPool::connect(&configuration.database.get_connection_string())
        .await
        .expect("Cant get DB connection");
    let address = format!("127.0.0.1:{}",configuration.application_port);
    let listener = TcpListener::bind(address).expect("Unable to bound the address");
    run(listener, connection_pool)?.await
}
