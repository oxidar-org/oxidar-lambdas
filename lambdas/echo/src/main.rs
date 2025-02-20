use lambda_http::{run, tower, Error, Response};
mod error;
mod handler;

use ::tracing_handler::initialize_tracing;
use handler::function_handler;
use tower::service_fn;

const LAMBDA_NAME: &str = "echo";

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    run(service_fn(|request| async {
        let response = function_handler(request).await;

        Ok::<lambda_http::Response<std::string::String>, Error>(match response {
            Ok(r) => r,
            Err(e) => Response::<String>::from(e),
        })
    }))
    .await
}
