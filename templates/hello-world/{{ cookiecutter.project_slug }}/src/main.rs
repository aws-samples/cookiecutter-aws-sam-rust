use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    Request,
    Response,
    IntoResponse,
};

#[tokio::main]
async fn main() {
    lambda_runtime::run(handler(hello_world)).await?;

    Ok(())
}

/// Sample pure Lambda function
async fn hello(
    request: Request,
    _: Context,
) -> Result<impl IntoResponse, Error> {
    Response::builder()
        .status(200)
        .body("Hello, World!".to_string())
}

#[test]
fn test_hello() {
    let request = Request::default();
    let response = hello(request, Context::default()).unwrap();
    assert_eq!(response.body, "Hello, World!");
}