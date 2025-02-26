use std::future::Future;

use lambda_http::RequestPayloadExt;
use lambda_http::{service_fn, Body, Error, Request, Response};
use serde::de::DeserializeOwned;

use crate::error::HttpError;

pub async fn run<'a, R, P, F, I, Fut>(f: F, persisted: &'a P) -> Result<(), Error>
where
    lambda_http::Body: From<R>,
    P: Send + Sync,
    I: DeserializeOwned + Send,
    F: Fn(I, &'a P) -> Fut + Sync,
    Fut: Future<Output = Result<Response<R>, HttpError>> + Send + Sync,
{
    lambda_http::run(service_fn(async |request: Request| {
        let input: Result<Option<I>, HttpError> = request
            .payload::<I>()
            .map_err(|e| HttpError::InvalidRequestBody(format!("{e}")));

        Ok::<lambda_http::Response<Body>, Error>(match input {
            Ok(None) => Response::<String>::from(HttpError::EmptyRequestBody).map(Body::Text),
            Ok(Some(input)) => {
                let response = f(input, persisted).await;

                match response {
                    Ok(r) => r.map(Body::from),
                    Err(e) => (Response::<String>::from(e)).map(Body::Text),
                }
            }
            Err(e) => Response::<String>::from(e).map(Body::Text),
        })
    }))
    .await
}
