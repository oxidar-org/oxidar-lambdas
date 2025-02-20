use lambda_runtime::{tower, Error, Runtime};
mod handler;

use ::tracing_handler::initialize_tracing;
use handler::function_handler;
use tower::service_fn;

const LAMBDA_NAME: &str = "add_path";

pub struct PersistedMemory {
    redis_client: redis::Client,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let tracing_provider = initialize_tracing(LAMBDA_NAME);

    let persisted = PersistedMemory {
        redis_client: redis::Client::open("redis://127.0.0.1").expect("could not connect to redis"),
    };

    /*
        let runtime = Runtime::new(service_fn(|d| function_handler(d, &persisted)))
            .layer(otel_layer(&tracing_provider));
    runtime.run().await?;
    */

    Ok(())
}
