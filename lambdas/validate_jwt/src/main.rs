mod error;
mod generic_handler;
mod models;

use ::tracing::{initialize_tracing, otel_layer};
use generic_handler::function_handler;
use jsonwebtoken::jwk::JwkSet;
use lambda_runtime::{tower, tracing, Error, Runtime};
use tower::service_fn;

pub struct PersistedMemory {
    redis_client: redis::Client,
    jwks: JwkSet,
}

const DEFAULT_LOG_LEVEL: &str = "INFO";
const APP_NAME: &str = "lambda_test";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let tracing_provider = initialize_tracing(APP_NAME);

    tracing::info!("initializing lambda...");

    let jwks: JwkSet = serde_json::from_str(
        &reqwest::get("http://127.0.0.1:8080/.well-known/jwks.json")
            .await
            .expect("could not get JWKS")
            .text()
            .await
            .expect("invalid jwks"),
    )
    .expect("unable to parse jwks");

    let persisted = PersistedMemory {
        redis_client: redis::Client::open("redis://127.0.0.1").expect("could not connect to redis"),
        jwks,
    };

    // run(service_fn(|d| function_handler(d, &persisted))).await
    // Initialize the Lambda runtime and add OpenTelemetry tracing
    let runtime = Runtime::new(service_fn(|d| function_handler(d, &persisted)))
        .layer(otel_layer(&tracing_provider));

    runtime.run().await?;
    Ok(())
}
