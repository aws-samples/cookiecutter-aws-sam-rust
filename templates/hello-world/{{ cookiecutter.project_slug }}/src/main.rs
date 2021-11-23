use lambda_http::{
    handler,
    lambda_runtime::{self, Context, Error},
    Request,
    Response,
    IntoResponse,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(hello)).await?;

    Ok(())
}

/// Sample pure Lambda function
async fn hello(
    _request: Request,
    _context: Context,
) -> Result<impl IntoResponse, Error> {
    Ok(Response::builder()
        .status(200)
        .body("Hello, World!".to_string())?
    )
}

#[test]
fn test_hello() {
    let request = Request::default();
    let response = hello(request, Context::default()).unwrap();
    assert_eq!(response.body, "Hello, World!");
}