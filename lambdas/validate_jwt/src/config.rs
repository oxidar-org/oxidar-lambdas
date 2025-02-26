use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub redis_url: String,
    pub jwks_url: String,
}
