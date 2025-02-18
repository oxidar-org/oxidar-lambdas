use lambda_runtime::{tracing, Error, LambdaEvent};
use redis::Commands;
use serde::Deserialize;

use crate::PersistedMemory;

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    role: String,
    path: String,
}

pub(crate) async fn function_handler(
    event: LambdaEvent<IncomingMessage>,
    persisted: &PersistedMemory,
) -> Result<(), Error> {
    let IncomingMessage { role, path } = event.payload;

    let mut redis = persisted.redis_client.get_connection()?;

    tracing::info!("added permissiton to access {path} to role {role}");

    redis.sadd(role, path)?;

    Ok(())
}
