use lambda_runtime::{
    run, service_fn,
    tracing::{self, instrument::WithSubscriber},
    Error,
};
mod generic_handler;
use generic_handler::function_handler;

#[derive(Debug)]
pub struct PersistedMemory {
    dynamodb_client: aws_sdk_dynamodb::Client,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    // let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .test_credentials()
        .endpoint_url("http://localhost:8000")
        .region("us-west-2")
        .load()
        .await;

    tracing::info!("initializing lambda...");
    let client = aws_sdk_dynamodb::Client::new(&config);

    let persisted = PersistedMemory {
        dynamodb_client: client,
    };

    run(service_fn(|d| function_handler(d, &persisted))).await
}
