use std::future::Future;

use lambda_http::{lambda_runtime::streaming::Body, service_fn, Error, Request, Response};

use crate::error::HttpError;

/*
pub async fn run<R: From<Body>, P>(
    f: impl Fn(Request, &P) -> impl Future<Output = Result<Response<R>, HttpError>>,
    persisted: &P,
) -> Result<(), Error> {
    lambda_http::run(service_fn(|request| async {
        let response = f(request, &persisted).await;

        Ok::<lambda_http::Response<Body>, Error>(match response {
            Ok(r) => r.map(Body::from),
            Err(e) => (Response::<String>::from(e)).map(Body::Text),
        })
    }))
    .await
}*/
