use http::error::HttpError;
use lambda_http::http::StatusCode;
use lambda_http::{tracing, Response};
use redis::Commands;

use crate::input::IncomingMessage;
use crate::PersistedMemory;

pub(crate) async fn function_handler(
    input: IncomingMessage,
    persisted: &PersistedMemory,
) -> Result<Response<()>, HttpError> {
    let IncomingMessage { role, path } = input;

    let mut redis = persisted
        .redis_client
        .get_connection()
        .map_err(|e| HttpError::Unknown(Box::new(e)))?;

    tracing::info!("added permissiton to access {path} to role {role}");

    redis
        .sadd::<String, String, String>(role, path)
        .map_err(|e| HttpError::Unknown(Box::new(e)))?;

    let response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(())
        .map_err(|e| HttpError::Unknown(Box::new(e)))?;

    Ok(response)
}
