use lambda_runtime::{tracing, Error, LambdaEvent};
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::PersistedMemory;

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    role: String,
    path: String,
}

#[derive(Serialize)]
pub(crate) struct OutgoingMessage {
    req_id: String,
    msg: String,
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

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};

    #[tokio::test]
    async fn test_generic_handler() {
        let event = LambdaEvent::new(
            IncomingMessage {
                command: "test".to_string(),
            },
            Context::default(),
        );
        let response = function_handler(event).await.unwrap();
        assert_eq!(response.msg, "Command test.");
    }
    */
}
