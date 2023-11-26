use askama::Template;
use aws_config::{load_from_env, SdkConfig};
use aws_sdk_dynamodb::Client;
use backend::utils::add_poll::add_poll;
use backend::utils::add_poll::Poll;
use backend::utils::html_response::build;
use lambda_http::request::from_str;
use lambda_http::RequestExt;
use lambda_http::RequestPayloadExt;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use lambda_runtime::Context;
use log::error;
use log::Level;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Default)]
struct FormData {
    title: String,
    description: String,
}

#[derive(Template)]
#[template(path = "add-question.html")]
struct AddQuestionTemplate {}

pub async fn add_question_handler(event: Request) -> Result<Response<Body>, Error> {
    let template = AddQuestionTemplate {};

    let formdata: FormData = event
        .payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    log::info!("Form data: {:?}", formdata);

    let poll = Poll {
        pollId: Uuid::new_v4().to_string(),
        title: formdata.title.to_string(),
        description: formdata.description.to_string(),
    };

    log::info!("Poll: {:?}", poll);

    let result = add_poll(
        &Client::new(&load_from_env().await),
        poll,
        &"polls".to_string(),
    )
    .await?;

    log::info!("Result: {:?}", result);

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
    run(service_fn(add_question_handler)).await
}
