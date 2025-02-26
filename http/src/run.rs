#[macro_export]
macro_rules! run {
    ($f: expr, $persisted: ident, $input: ty) => {
        lambda_http::run(lambda_http::tower::service_fn(
            async |request: lambda_http::Request| {
                use lambda_http::RequestPayloadExt;
                let input: Result<Option<$input>, $crate::error::HttpError> = request
                    .payload::<$input>()
                    .map_err(|e| $crate::error::HttpError::InvalidRequestBody(format!("{e}")));

                Ok::<lambda_http::Response<lambda_http::Body>, lambda_http::Error>(match input {
                    Ok(None) => lambda_http::Response::<String>::from(
                        $crate::error::HttpError::EmptyRequestBody,
                    )
                    .map(lambda_http::Body::Text),
                    Ok(Some(input)) => {
                        let response = $f(input, &($persisted)).await;

                        match response {
                            Ok(r) => r.map(lambda_http::Body::from),
                            Err(e) => (lambda_http::Response::<String>::from(e))
                                .map(lambda_http::Body::Text),
                        }
                    }
                    Err(e) => lambda_http::Response::<String>::from(e).map(lambda_http::Body::Text),
                })
            },
        ))
        .await
    };
}
