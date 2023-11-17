use askama::Template;
use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

#[derive(Template)]
#[template(path = "helloworld.html")]
struct HelloWorldTemplate {
    name: String,
    number: u32,
}

pub async fn good_morning_handler(event: Request) -> Result<Response<Body>, Error> {
    let query_params = event.query_string_parameters_ref();

    let name = query_params
        .and_then(|params| params.first("name"))
        .unwrap_or("world")
        .to_string();

    let template = HelloWorldTemplate { name, number: 99 };

    match template.render() {
        Ok(html_content) => {
            let resp = Response::builder()
                .status(200)
                .header("content-type", "text/html")
                .body(html_content.into())
                .map_err(Box::new)?;
            Ok(resp)
        }
        Err(e) => {
            // Log error here if necessary
            Err(Box::new(e))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(good_morning_handler)).await
}
