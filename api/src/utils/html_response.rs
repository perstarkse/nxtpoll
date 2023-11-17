use lambda_http::{http::HeaderValue, http::StatusCode, Body, Response};
use std::error::Error;

pub fn build(
    html_content: String,
) -> Result<Response<Body>, Box<dyn Error + Send + Sync + 'static>> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "text/html")
        .header("Access-Control-Allow-Origin", HeaderValue::from_static("*"))
        .header(
            "Access-Control-Allow-Methods",
            HeaderValue::from_static("GET, POST, OPTIONS"),
        )
        .header(
            "Access-Control-Allow-Headers",
            HeaderValue::from_static("Content-Type"),
        )
        .body(html_content.into())
        .expect("Failed to render response");

    Ok(response)
}
