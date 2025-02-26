mod handler;
mod input;

use crate::handler::function_handler;
use ::tracing_handler::initialize_tracing;
use http::run::run;
use lambda_http::Error;

const LAMBDA_NAME: &str = "echo";

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    run(function_handler, &()).await
}
