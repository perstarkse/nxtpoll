use askama::Template;
use backend::utils::html_response::build;
use lambda_http::{service_fn, Body, Error, Request, Response};
use log::{error, Level};

#[derive(Template)]
#[template(path = "create-poll.html")]
struct CreatePollTemplate {}

pub async fn create_poll_handler(event: Request) -> Result<Response<Body>, Error> {
    let template = CreatePollTemplate {};

    match template.render() {
        Ok(html_content) => build(html_content),
        Err(e) => {
            error!("Error rendering template: {:?}", e);
            Err(Box::new(e))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    simple_logger::init_with_level(Level::Info).unwrap();
    lambda_http::run(service_fn(create_poll_handler)).await
}
