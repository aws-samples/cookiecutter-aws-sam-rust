use lambda_http::{service_fn, Error, IntoResponse, Request, Response};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_http::run(service_fn(hello)).await?;

    lambda_http::run(service_fn(|event: Request| hello(event))).await?;

    Ok(())
}

/// Sample pure Lambda function
async fn hello(_request: Request) -> Result<impl IntoResponse, Error> {
    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body("Hello, World!".to_string())?)
}

#[tokio::test]
async fn test_hello() {
    let request = Request::default();
    let response = hello(request).await.unwrap().into_response();
    assert_eq!(
        response.body(),
        &lambda_http::Body::Text("Hello, World!".to_string())
    );
}
