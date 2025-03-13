use std::time::Duration;

use jsonwebtoken::{
    decode, decode_header, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation,
};
use lambda_runtime::{tracing, LambdaEvent};
use redis::Commands;
use serde::Deserialize;

use crate::{
    error::ErrorResponse,
    models::{
        claims::Claims,
        response::{Effect, Response},
    },
    PersistedMemory,
};

#[derive(Deserialize)]
struct Headers {
    #[serde(
        alias = "Authorization",
        alias = "AUTHORIZATION",
        alias = "Authorization"
    )]
    authorization: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IncomingMessage {
    path: String,
    headers: Headers,
    method_arn: String,
}

pub(crate) async fn function_handler(
    event: LambdaEvent<IncomingMessage>,
    persisted: &PersistedMemory,
) -> Result<String, ErrorResponse> {
    let jwt = &event.payload.headers.authorization;

    tracing::info!("processing {jwt}...");
    // Decode the header
    let header = decode_header(jwt)?;

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

        decode::<Claims>(jwt, key, &Validation::new(Algorithm::RS256))?
    } else {
        return Err(ErrorResponse::JwtKeyNotFoundInJwks(kid));
    };

    let mut redis = persisted
        .redis_client
        .get_connection_with_timeout(Duration::from_secs(5))?;
    let path_is_permited: bool =
        redis.sismember(token.claims.roles[0].as_str(), &event.payload.path)?;

    let effect = if path_is_permited {
        tracing::info!(
            "{}: granting acess {}",
            &token.claims.sub,
            &event.payload.path
        );
        Effect::Allow
    } else {
        tracing::warn!(
            "permission denied: the user {} is trying to access the restricted resource {}",
            token.claims.sub,
            &event.payload.path
        );

        Effect::Deny
    };

    println!("5");
    Ok(Response::with_params(
        effect,
        event.payload.method_arn,
        token.claims.sub,
    ))
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
