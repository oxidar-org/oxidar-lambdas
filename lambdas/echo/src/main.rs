mod handler;
mod input;

use crate::handler::function_handler;
use ::tracing_handler::initialize_tracing;
use http::run;
use input::IncomingMessage;
use lambda_http::Error;

const LAMBDA_NAME: &str = "echo";

#[tokio::main]
async fn main() -> Result<(), Error> {
    initialize_tracing(LAMBDA_NAME);

    let persisted = ();
    run!(function_handler, persisted, IncomingMessage)
}
