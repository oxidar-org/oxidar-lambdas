use crate::error::EchoError;
use lambda_http::{http::StatusCode, tracing, IntoResponse, Request, RequestPayloadExt, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    message: String,
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<String>, EchoError> {
    let IncomingMessage { message } = event
        .payload::<IncomingMessage>()
        .map_err(|e| EchoError::InvalidRequestBody(format!("{e}")))?
        .ok_or(EchoError::EmptyRequestBody)?;

    tracing::info!(r#"echoing {message}"#);

    // Represents an HTTP response
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": format!("Hello, {}!", message),
            })
            .to_string(),
        )
        .map_err(|e| EchoError::Unknown(e.into()))?;

    Ok(response)
}
