use askama::Template;
use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;
use backend::models::poll::Poll;
use backend::utils::html_response::build;
use backend::utils::save_question::save_question_fn;
use backend::utils::save_question::SaveQuestion;
use lambda_http::RequestPayloadExt;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use log::error;
use log::Level;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
struct FormData {
    poll_id: String,
    question: String,
    alternatives: Vec<String>,
}

#[derive(Template)]
#[template(path = "saved-questions-list.html")]
struct QuestionListTemplate {
    questions: Vec<String>, // Array of question strings
}

pub async fn save_question_handler(event: Request) -> Result<Response<Body>, Error> {
    let formdata: FormData = event
        .payload()
        .unwrap_or_else(|_parse_err| None)
        .unwrap_or_default();

    let poll_id = formdata.poll_id.clone();
    let question = formdata.question.clone();
    let alternatives = formdata.alternatives.clone();

    let save_question = SaveQuestion {
        poll_id,
        question,
        alternatives,
    };

    let client = Client::new(&load_from_env().await);

    let result = save_question_fn(&client, save_question, &"polls".to_string()).await?;

    log::info!("Result: {:?}", result);

    let retrieved_poll: Poll = get_poll(formdata.poll_id.clone()).await;

    let template = QuestionListTemplate {
        questions: retrieved_poll.questions,
    };

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
    run(service_fn(save_question_handler)).await
}
