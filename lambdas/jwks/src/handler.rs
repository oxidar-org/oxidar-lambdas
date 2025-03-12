use lambda_http::{http::StatusCode, Response};

const JWKS_TEXT: &str = r#"{"keys":[{"alg":"RS256","e":"AQAB","key_ops":null,"kid":"r3gCBZpAqIbYlLtm9LlOWftLVXLOE5u6m8xemvgbeHI","kty":"RSA","n":"69OzDG8xIMTonqc9Vf4a9akK5zm6fBVq-S-RJT5G2oBkb6QWpAlNEZio7YiV1Vs9-4GEPI50i3x3cO2K5Cfum8eLj1dLe9zurQ0UADAtQcYuWGy7Q7O9W3sQPkQhe7IRIkDtltdxxI25bLUbB6WieIlPjjk9UygqgITWywdsoadBD5i96n17ASMZRDxPz5UJj13ntuYOmeenJfdaozW0TrBBrYbJotUlQIl4Joq0pbJbofr4qpu1f561dN8HmpwuKVjrKaxFJ121Wy2HYoQhQxMzvXItfVclnBJCPS3sxyzQhS0qVWJJyF1Cd2F-ptJRQ_85-mzksezKSz-3vfI-bw","use":"sig"}]}"#;

pub(crate) async fn function_handler() -> Response<String> {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(JWKS_TEXT.to_string())
        .expect("unexpected error")
}
