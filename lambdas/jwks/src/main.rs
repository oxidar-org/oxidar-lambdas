mod handler;

use crate::handler::function_handler;
use ::tracing_handler::initialize_tracing;
use lambda_http::{service_fn, Error, Response};

const LAMBDA_NAME: &str = "jwks";

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    lambda_http::run(service_fn(async |_| {
        Ok::<Response<String>, Error>(function_handler().await)
    }))
    .await
}
