use lambda_runtime::{run, service_fn, tracing, Error};
mod generic_handler;
use generic_handler::function_handler;

pub struct PersistedMemory {
    redis_client: redis::Client,
    http_client: reqwest::Client,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let persisted = PersistedMemory {
        redis_client: redis::Client::open("redis://127.0.0.1").expect("could not connect to redis"),
        http_client: reqwest::Client::new(),
    };

    run(service_fn(|d| function_handler(d, &persisted))).await
}
