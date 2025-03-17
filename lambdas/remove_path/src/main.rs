mod config;
mod handler;
mod input;

use ::tracing_handler::initialize_tracing;
use config::Config;
use handler::function_handler;
use http::run::run;
use lambda_http::Error;
use redis::aio::ConnectionManager;

const LAMBDA_NAME: &str = "add_path";

pub struct PersistedMemory {
    redis: ConnectionManager,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    let config = envy::from_env::<Config>().expect("unable to load configuration");

    let redis = redis::Client::open(config.redis_url.to_string())
        .expect("could create redis client")
        .get_connection_manager()
        .await
        .expect("could not create connection manager");

    let persisted = PersistedMemory { redis };

    run(function_handler, &persisted).await
}
