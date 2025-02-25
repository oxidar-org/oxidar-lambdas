use lambda_http::Body;

pub type Empty = ();

pub struct Json(serde_json::Value);

impl From<Json> for Body {
    fn from(value: Json) -> Self {
        Body::Text(value.0.to_string())
    }
}

impl From<serde_json::Value> for Json {
    fn from(value: serde_json::Value) -> Self {
        Self(value)
    }
}
