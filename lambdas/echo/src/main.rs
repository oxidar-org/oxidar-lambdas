mod error;
mod handler;

use ::tracing_handler::initialize_tracing;
use handler::function_handler;
use lambda_http::{run, tower, Body, Error, Response};
use tower::service_fn;

const LAMBDA_NAME: &str = "echo";

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    run(service_fn(|request| async {
        let response = function_handler(request).await;

        Ok::<lambda_http::Response<Body>, Error>(match response {
            Ok(r) => r.map(Body::from),
            Err(e) => (Response::<String>::from(e)).map(Body::Text),
        })
    }))
    .await
}
