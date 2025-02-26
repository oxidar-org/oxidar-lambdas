mod handler;
mod input;

use ::tracing_handler::initialize_tracing;
use handler::function_handler;
use http::run;
use input::IncomingMessage;
use lambda_http::Error;

const LAMBDA_NAME: &str = "add_path";

pub struct PersistedMemory {
    redis_client: redis::Client,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    let persisted = PersistedMemory {
        redis_client: redis::Client::open("redis://127.0.0.1").expect("could not connect to redis"),
    };

    run!(function_handler, persisted, IncomingMessage)
}
