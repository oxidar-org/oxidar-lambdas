use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct IncomingMessage {
    pub message: String,
}
