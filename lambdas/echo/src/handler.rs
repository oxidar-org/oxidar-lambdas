use http::{error::HttpError, response::Json};
use lambda_http::{http::StatusCode, tracing, Request, RequestPayloadExt, Response};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    message: String,
}

pub(crate) async fn function_handler(event: Request) -> Result<Response<Json>, HttpError> {
    let IncomingMessage { message } = event
        .payload::<IncomingMessage>()
        .map_err(|e| HttpError::InvalidRequestBody(format!("{e}")))?
        .ok_or(HttpError::EmptyRequestBody)?;

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
        .map_err(|e| HttpError::Unknown(e.into()))?;

    Ok(response)
}
