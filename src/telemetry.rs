use tracing::{Dispatch, Subscriber};
use tracing::dispatcher::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, Registry};

pub fn get_subscriber(
    name :String,
    env_filter:String
) -> impl Subscriber + Send +Sync{

    // get env Filter from environment variable or we can use  funtion input
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or(EnvFilter::new(env_filter));

    // add formatting layer to our telemetry system

    let formatting_layer = BunyanFormattingLayer::new(
        name.into(),
        std::io::stdout
    );

    //Construct a subscriber and return the subscriber
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber( subscriber : impl  Subscriber + Send + Sync){
    // Intialise tracing log
    LogTracer::init().expect(" Unable to intialise the  log tracer");
    set_global_default(Dispatch::from(subscriber)).expect(" Failed to set  tracing subscriber as global level");
}