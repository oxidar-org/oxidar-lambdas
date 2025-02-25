use http::error::HttpError;
use lambda_http::http::StatusCode;
use lambda_http::RequestPayloadExt;
use lambda_http::{tracing, Request, Response};
use redis::Commands;
use serde::Deserialize;

use crate::PersistedMemory;

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    role: String,
    path: String,
}

pub(crate) async fn function_handler(
    event: Request,
    persisted: &PersistedMemory,
) -> Result<Response<()>, HttpError> {
    let IncomingMessage { role, path } = event
        .payload::<IncomingMessage>()
        .map_err(|e| HttpError::InvalidRequestBody(format!("{e}")))?
        .ok_or(HttpError::EmptyRequestBody)?;

    let mut redis = persisted
        .redis_client
        .get_connection()
        .map_err(|e| HttpError::Unknown(e.into()))?;

    tracing::info!("added permissiton to access {path} to role {role}");

    redis
        .sadd::<String, String, String>(role, path)
        .map_err(|e| HttpError::Unknown(e.into()))?;

    let response = Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(())
        .map_err(|e| HttpError::Unknown(e.into()))?;

    Ok(response)
}
