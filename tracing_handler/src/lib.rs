use opentelemetry_sdk::trace;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{prelude::*, EnvFilter};

const DEFAULT_LOG_LEVEL: &str = "INFO";

pub fn initialize_tracing(app_name: &'static str) -> trace::TracerProvider {
    let formatting_layer = BunyanFormattingLayer::new(app_name.into(), std::io::stdout);
    let tracer_provider = trace::TracerProvider::default();

    // Set up link between OpenTelemetry and tracing crate
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_LOG_LEVEL)),
        )
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    tracer_provider
}
