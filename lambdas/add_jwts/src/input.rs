use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    pub role: String,
    pub path: String,
}
