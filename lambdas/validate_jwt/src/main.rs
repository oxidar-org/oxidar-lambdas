mod error;
mod generic_handler;
mod models;

use generic_handler::function_handler;
use jsonwebtoken::jwk::JwkSet;
use lambda_runtime::{
    layers::{OpenTelemetryFaasTrigger, OpenTelemetryLayer as OtelLayer},
    tower, tracing, Error, Runtime,
};
use opentelemetry::trace::TracerProvider;
use opentelemetry_sdk::trace;
use tower::service_fn;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::*, EnvFilter};

pub struct PersistedMemory {
    redis_client: redis::Client,
    jwks: JwkSet,
}

const DEFAULT_LOG_LEVEL: &str = "INFO";
const APP_NAME: &str = "lambda_test";

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up OpenTelemetry tracer provider that writes spans to stdout for debugging purposes
    // let exporter = opentelemetry_stdout::SpanExporter::default();
    let formatting_layer = BunyanFormattingLayer::new(APP_NAME.into(), std::io::stdout);
    let tracer_provider = trace::TracerProvider::builder()
        //.with_batch_exporter(exporter, runtime::Tokio)
        .build();

    // Set up link between OpenTelemetry and tracing crate
    tracing_subscriber::registry()
        .with(tracing_opentelemetry::OpenTelemetryLayer::new(
            tracer_provider.tracer(APP_NAME),
        ))
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL)),
        )
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

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
    let runtime = Runtime::new(service_fn(|d| function_handler(d, &persisted))).layer(
        // Create a tracing span for each Lambda invocation
        OtelLayer::new(|| {
            // Make sure that the trace is exported before the Lambda runtime is frozen
            tracer_provider.force_flush();
        })
        // Set the "faas.trigger" attribute of the span to "pubsub"
        .with_trigger(OpenTelemetryFaasTrigger::Http),
    );

    runtime.run().await?;
    Ok(())
}
