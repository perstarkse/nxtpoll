use askama::Template;
use lambda_http::{http::StatusCode, run, service_fn, Body, Error, Request, RequestExt, Response};
use log::{error, info}; // Import logging macros

#[derive(Template)]
#[template(path = "helloworld.html")]
struct HelloWorldTemplate {
    name: String,
    number: u32,
}

pub async fn hello_world_handler(event: Request) -> Result<Response<Body>, Error> {
    // Log the incoming request
    info!("Received request: {:?}", event);

    let query_params = event.query_string_parameters_ref();

    let name = query_params
        .and_then(|params| params.first("name"))
        .unwrap_or("world")
        .to_string();

    let template = HelloWorldTemplate { name, number: 45 };

    match template.render() {
        Ok(html_content) => {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html")
                .body(html_content.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        Err(e) => {
            error!("Error rendering template: {:?}", e);
            Err(Box::new(e))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(log::Level::Info).unwrap(); // Initialize the logger
    run(service_fn(hello_world_handler)).await
}
