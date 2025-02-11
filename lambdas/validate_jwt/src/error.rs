use lambda_runtime::Diagnostic;

#[derive(thiserror::Error, Debug)]
pub enum ErrorResponse {
    #[error("invalid jwt: {0}")]
    InvalidJwt(String),

    #[error("jwt key {0} not found in JWKS")]
    JwtKeyNotFoundInJwks(String),

    #[error("jwt key id not present in token")]
    JwtKeyIdNotPresent,

    #[error("role not found")]
    RoleNotFound,

    #[error("managing token: {0}")]
    ManagingToken(#[from] jsonwebtoken::errors::Error),

    #[error("cache error: {0}")]
    CacheError(#[from] redis::RedisError),
}

impl From<ErrorResponse> for Diagnostic {
    fn from(value: ErrorResponse) -> Self {
        Self {
            error_type: "AuthError".to_owned(),
            error_message: value.to_string(),
        }
    }
}
