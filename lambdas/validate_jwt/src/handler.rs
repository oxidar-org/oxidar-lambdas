use jsonwebtoken::{
    decode, decode_header, jwk::AlgorithmParameters, Algorithm, DecodingKey, Validation,
};
use lambda_runtime::{tracing, LambdaEvent};
use redis::{aio::ConnectionLike, AsyncCommands};
use serde::Deserialize;

use crate::{
    error::ErrorResponse,
    models::{
        claims::{Claims, Role},
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

pub(crate) async fn function_handler<R>(
    event: LambdaEvent<IncomingMessage>,
    persisted: &PersistedMemory<R>,
) -> Result<serde_json::Value, ErrorResponse>
where
    R: ConnectionLike + Clone + AsyncCommands,
{
    match validate_jwt(&event.payload, persisted).await {
        Ok(r) => Ok(r),
        Err(e) => match e {
            ErrorResponse::InvalidJwt(_)
            | ErrorResponse::ManagingToken(_)
            | ErrorResponse::JwtKeyNotFoundInJwks(_)
            | ErrorResponse::JwtKeyIdNotPresent => {
                tracing::warn!("{e}");
                Ok(Response::with_params(
                    Effect::Deny,
                    &event.payload.method_arn,
                    "user",
                ))
            }
            ErrorResponse::CacheError(_) => {
                tracing::error!("{e}");
                Err(e)
            }
        },
    }
}

async fn validate_jwt<R>(
    event: &IncomingMessage,
    persisted: &PersistedMemory<R>,
) -> Result<serde_json::Value, ErrorResponse>
where
    R: ConnectionLike + Clone + AsyncCommands,
{
    let IncomingMessage {
        path,
        headers,
        method_arn,
    } = event;
    let jwt = &headers.authorization[7..];

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

    if token.claims.roles[0] == Role::Admin {
        return Ok(Response::with_params(
            Effect::Allow,
            method_arn,
            &token.claims.sub,
        ));
    }

    let path_is_permited: bool = persisted
        .redis
        .clone()
        .sismember(token.claims.roles[0].as_str(), path)
        .await?;

    let effect = if path_is_permited {
        tracing::info!("{}: granting acess {}", &token.claims.sub, path);
        Effect::Allow
    } else {
        tracing::warn!(
            "permission denied: the user {} is trying to access the restricted resource {}",
            token.claims.sub,
            path
        );

        Effect::Deny
    };

    Ok(Response::with_params(effect, method_arn, &token.claims.sub))
}

#[cfg(test)]
mod tests {
    use super::*;
    use lambda_runtime::{Context, LambdaEvent};
    use redis_test::MockRedisConnection;

    const JWKS: &str = r#"{"keys":[{"alg":"RS256","e":"AQAB","key_ops":null,"kid":"r3gCBZpAqIbYlLtm9LlOWftLVXLOE5u6m8xemvgbeHI","kty":"RSA","n":"69OzDG8xIMTonqc9Vf4a9akK5zm6fBVq-S-RJT5G2oBkb6QWpAlNEZio7YiV1Vs9-4GEPI50i3x3cO2K5Cfum8eLj1dLe9zurQ0UADAtQcYuWGy7Q7O9W3sQPkQhe7IRIkDtltdxxI25bLUbB6WieIlPjjk9UygqgITWywdsoadBD5i96n17ASMZRDxPz5UJj13ntuYOmeenJfdaozW0TrBBrYbJotUlQIl4Joq0pbJbofr4qpu1f561dN8HmpwuKVjrKaxFJ121Wy2HYoQhQxMzvXItfVclnBJCPS3sxyzQhS0qVWJJyF1Cd2F-ptJRQ_85-mzksezKSz-3vfI-bw","use":"sig"}]}"#;
    const ARN: &str = "arn:aws:execute-api:us-east-1:123456789012:ymy8tbxw7b/*/GET/some_path";

    fn get_persisted_memory() -> PersistedMemory<MockRedisConnection> {
        PersistedMemory {
            redis: redis_test::MockRedisConnection::new(vec![]),
            jwks: serde_json::from_str(JWKS).expect("unable to parse jwks"),
        }
    }

    #[tokio::test]
    async fn err_jwt_key_not_found_in_jwks() {
        const INVALID_KID_JWT: &str = "Bearer ewogICJhbGciOiAiUlMyNTYiLAogICJraWQiOiAiaW52YWxpZF9raWQiLAogICJ0eXAiOiAiSldUIgp9.eyJleHAiOjE3NDIyNDIzNjgsInJvbGVzIjpbInN1cGVyX3VzZXIiXSwic3ViIjoibmRyd2ZnbHZtaUBnbWFpbC5jb20ifQ.wfYlJQqGaiEEHviYau_QK1VHbiPA3Vi6EKEbpidC2hLe0aJ-ICKSYf5FPXxKebI_Yy9I0mHFAHo4BrYy4IoAtgbXMfRUGf1M36cIU4yI1pAu-ECGopgj7flIw1azajh-Xf1JuxlyhFDOyM_FIOqxt_dJm5ZA3t9O-hRVCzKOCrRYRRzDHv1cqJOSPaUmdNW0GTad1oK5eipbBB3k9Rug80aMGamV8szpFxaG1Om92lSNDmeXKPsdFRuHDW9-_AOgrDeeh_zyv-dLN27Q_jN8l0NKg8q8V8WGeIzkITt4kKTp6tYwk5K8sFMCxhLTvuQGvlUaKWEpS_lIZdlasultwA";
        let event = LambdaEvent {
            payload: IncomingMessage {
                path: "/some_path".to_string(),
                headers: Headers {
                    authorization: INVALID_KID_JWT.to_string(),
                },
                method_arn: ARN.to_string(),
            },
            context: Context::default(),
        };

        let persisted = get_persisted_memory();

        let result = validate_jwt(&event.payload, &persisted).await.unwrap_err();

        assert_eq!(
            result,
            ErrorResponse::JwtKeyNotFoundInJwks("invalid_kid".to_string())
        );
    }

    #[tokio::test]
    async fn err_jwt_kid_key_in_header_not_found() {
        const INVALID_KID_JWT: &str = "Bearer ewogICJhbGciOiAiUlMyNTYiLAogICJ0eXAiOiAiSldUIgp9.eyJleHAiOjE3NDIyNDIzNjgsInJvbGVzIjpbInN1cGVyX3VzZXIiXSwic3ViIjoibmRyd2ZnbHZtaUBnbWFpbC5jb20ifQ.wfYlJQqGaiEEHviYau_QK1VHbiPA3Vi6EKEbpidC2hLe0aJ-ICKSYf5FPXxKebI_Yy9I0mHFAHo4BrYy4IoAtgbXMfRUGf1M36cIU4yI1pAu-ECGopgj7flIw1azajh-Xf1JuxlyhFDOyM_FIOqxt_dJm5ZA3t9O-hRVCzKOCrRYRRzDHv1cqJOSPaUmdNW0GTad1oK5eipbBB3k9Rug80aMGamV8szpFxaG1Om92lSNDmeXKPsdFRuHDW9-_AOgrDeeh_zyv-dLN27Q_jN8l0NKg8q8V8WGeIzkITt4kKTp6tYwk5K8sFMCxhLTvuQGvlUaKWEpS_lIZdlasultwA";
        let event = LambdaEvent {
            payload: IncomingMessage {
                path: "/some_path".to_string(),
                headers: Headers {
                    authorization: INVALID_KID_JWT.to_string(),
                },
                method_arn: ARN.to_string(),
            },
            context: Context::default(),
        };

        let persisted = get_persisted_memory();

        let result = validate_jwt(&event.payload, &persisted).await.unwrap_err();

        assert_eq!(result, ErrorResponse::JwtKeyIdNotPresent);
    }
}
