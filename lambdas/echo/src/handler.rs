use lambda_runtime::{tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub(crate) struct IncomingMessage {
    message: String,
}

pub(crate) async fn function_handler(event: LambdaEvent<IncomingMessage>) -> Result<String, Error> {
    let IncomingMessage { message } = event.payload;

    tracing::info!(r#"echoing {message}"#);

    Ok(message)
}
