use jsonwebtoken::{
    decode, decode_header, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation,
};
use lambda_runtime::{tracing, LambdaEvent};
use redis::Commands;
use serde::{Deserialize, Serialize};

use crate::{error::ErrorResponse, models::claims::Claims, PersistedMemory};

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    token: String,
    path: String,
}

#[derive(Serialize)]
#[serde(untagged, rename = "snake_case")]
pub enum Response {
    AccessGranted,
    Forbidden,
}

pub(crate) async fn function_handler(
    event: LambdaEvent<IncomingMessage>,
    persisted: &PersistedMemory,
) -> Result<Response, ErrorResponse> {
    // Decode the header
    let header = decode_header(&event.payload.token)?;

    // Get the Verifying key by its id
    let kid = header.kid.ok_or(ErrorResponse::JwtKeyIdNotPresent)?;

    // Get the key from the JWKS
    let token = if let Some(jwk) = persisted.jwks.find(&kid) {
        let key = match &jwk.algorithm {
            AlgorithmParameters::RSA(key) => &DecodingKey::from_rsa_components(&key.n, &key.e)?,
            AlgorithmParameters::EllipticCurve(_) => todo!("elliptic curve not implemented"),
            AlgorithmParameters::OctetKey(_) => todo!("octet key not implemented"),
            AlgorithmParameters::OctetKeyPair(_) => {
                todo!("octet key pair not implemented")
            }
        };

        decode::<Claims>(
            &event.payload.token,
            key,
            &Validation::new(Algorithm::RS256),
        )?
    } else {
        return Err(ErrorResponse::JwtKeyNotFoundInJwks(kid));
    };

    let mut redis = persisted.redis_client.get_connection()?;
    let path_is_permited: bool =
        redis.sismember(token.claims.roles[0].as_str(), &event.payload.path)?;

    if path_is_permited {
        tracing::info!(
            "{}: granting acess {}",
            &token.claims.sub,
            &event.payload.path
        );
        Ok(Response::AccessGranted)
    } else {
        tracing::warn!(
            "permission denied: the user {} is trying to access the restricted resource {}",
            token.claims.sub,
            &event.payload.path
        );
        Ok(Response::Forbidden)
    }
}

/*
#[cfg(test)]
mod tests {
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
}
*/
