use askama::Template;
use backend::utils::html_response::build;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use log::error; // Import logging macros

#[derive(Template)]
#[template(path = "add-question.html")]
struct AddQuestionTemplate {}

pub async fn add_question_handler(event: Request) -> Result<Response<Body>, Error> {
    let template = AddQuestionTemplate {};

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
    run(service_fn(add_question_handler)).await
}
