mod config;
mod handler;
mod input;

use ::tracing_handler::initialize_tracing;
use config::Config;
use handler::function_handler;
use http::run::run;
use lambda_http::Error;

const LAMBDA_NAME: &str = "add_path";

pub struct PersistedMemory {
    redis_client: redis::Client,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    let config = envy::from_env::<Config>().expect("unable to load configuration");

    let persisted = PersistedMemory {
        redis_client: redis::Client::open(config.redis_url).expect("could not connect to redis"),
    };

    run(function_handler, &persisted).await
}
