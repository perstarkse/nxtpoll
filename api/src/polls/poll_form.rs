use askama::Template;
use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;
use backend::utils::add_poll::add_poll;
use backend::utils::add_poll::Poll;
use backend::utils::html_response::build;
use lambda_http::RequestPayloadExt;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
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
#[template(path = "poll_form.html")]
struct AddQuestionTemplate {
    poll_id: String,
    description: String,
    title: String,
}

pub async fn add_question_handler(event: Request) -> Result<Response<Body>, Error> {
    log::info!("Request payload: {:?}", event);

    let formdata: FormData = event
        .payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    let id = Uuid::new_v4().to_string();
    let title = formdata.title.to_string();
    let description = formdata.description.to_string();

    let template = AddQuestionTemplate {
        poll_id: id.clone(),
        description: description.clone(),
        title: title.clone(),
    };

    let poll = Poll {
        poll_id: id,
        title,
        description,
    };

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
