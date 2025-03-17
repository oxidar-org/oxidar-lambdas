use http::error::HttpError;
use lambda_http::http::StatusCode;
use lambda_http::{tracing, Response};
use redis::AsyncCommands;

use crate::input::IncomingMessage;
use crate::PersistedMemory;

pub(crate) async fn function_handler(
    input: IncomingMessage,
    persisted: &PersistedMemory,
) -> Result<Response<()>, HttpError> {
    let IncomingMessage { role, path } = input;

    persisted
        .redis
        .clone()
        .srem::<&str, &str, String>(&role, &path)
        .await
        .map_err(|e| HttpError::Unknown(Box::new(e)))?;

    tracing::info!("role {role} access to path {path} removed");

    let response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(())
        .map_err(|e| HttpError::Unknown(Box::new(e)))?;

    Ok(response)
}
