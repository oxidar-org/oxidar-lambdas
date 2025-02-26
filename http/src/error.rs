use lambda_http::{http::StatusCode, tracing, Response};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("empty request body")]
    EmptyRequestBody,

    #[error("invalid request body: {0}")]
    InvalidRequestBody(String),

    #[error("unknown error: {0:?}")]
    Unknown(Box<dyn std::error::Error + Send>),
}

impl From<HttpError> for Response<String> {
    fn from(value: HttpError) -> Self {
        let response = Response::builder().header("Content-Type", "application/json");

        match value {
            e @ HttpError::EmptyRequestBody | e @ HttpError::InvalidRequestBody(_) => response
                .status(StatusCode::BAD_REQUEST)
                .body(
                    json!({
                      "error": format!("{e}"),
                    })
                    .to_string(),
                )
                .unwrap(),
            HttpError::Unknown(error) => {
                tracing::error!("unknown error ocurred: {error}");

                response
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(
                        json!({
                          "error": "internal server error",
                        })
                        .to_string(),
                    )
                    .unwrap()
            }
        }
    }
}
