mod config;
mod error;
mod generic_handler;
mod models;

use ::tracing_handler::initialize_tracing;
use config::Config;
use generic_handler::function_handler;
use jsonwebtoken::jwk::JwkSet;
use lambda_runtime::{run, tower, tracing, Error};
use tower::service_fn;

pub struct PersistedMemory {
    redis_client: redis::Client,
    jwks: JwkSet,
}

const APP_NAME: &str = "lambda_test";

#[tokio::main]
async fn main() -> Result<(), Error> {
    //let tracing_provider = initialize_tracing(APP_NAME);
    initialize_tracing(APP_NAME);

    tracing::info!("initializing lambda...");

    tracing::info!("loading configuration...");
    let config = envy::from_env::<Config>().expect("unable to load configuration");

    tracing::info!("retrieving JWKS...");
    let jwks: JwkSet = serde_json::from_str(
        &reqwest::get(config.jwks_url.to_string())
            .await
            .expect("could not get JWKS")
            .text()
            .await
            .expect("invalid jwks"),
    )
    .expect("unable to parse jwks");

    tracing::info!("creating redis client...");
    let persisted = PersistedMemory {
        redis_client: redis::Client::open(config.redis_url.to_string())
            .expect("could not connect to redis"),
        jwks,
    };

    tracing::info!("listening for requests...");
    run(service_fn(|d| async {
        match function_handler(d, &persisted).await {
            Ok(r) => Ok(r),
            Err(e) => {
                tracing::error!("{e}");
                Err(e)
            }
        }
    }))
    .await
}
