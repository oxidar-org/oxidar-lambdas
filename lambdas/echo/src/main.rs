use lambda_runtime::{tower, Error, Runtime};
mod handler;

use ::tracing_handler::{initialize_tracing, otel_layer};
use handler::function_handler;
use tower::service_fn;

const LAMBDA_NAME: &str = "echo";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let tracing_provider = initialize_tracing(LAMBDA_NAME);

    let runtime = Runtime::new(service_fn(function_handler)).layer(otel_layer(&tracing_provider));

    runtime.run().await?;

    Ok(())
}
