use http::{error::HttpError, response::Json};
use lambda_http::{http::StatusCode, tracing, Response};
use serde_json::json;

use crate::input::IncomingMessage;

pub(crate) async fn function_handler(
    input: IncomingMessage,
    _: &(),
) -> Result<Response<Json>, HttpError> {
    let IncomingMessage { message } = input;

    tracing::info!(r#"echoing {message}"#);

    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(
            json!({
              "message": format!("Hello, {}!", message),
            })
            .into(),
        )
        .map_err(|e| HttpError::Unknown(Box::new(e)))?;

    Ok(response)
}
