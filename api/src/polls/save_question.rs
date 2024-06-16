use askama::Template;
use aws_config::load_from_env;
use aws_sdk_dynamodb::model::{AttributeValue, GetItemInput};
use aws_sdk_dynamodb::Client;
use backend::utils::html_response::build;
use backend::utils::save_question::save_question;
use backend::utils::save_question::SaveQuestion;
use lambda_http::RequestPayloadExt;
use lambda_http::{run, service_fn, Body, Error, Request, Response};
use log::error;
use log::Level;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug, Default)]
struct FormData {
    poll_id: String,
    question: String,
    alternatives: Vec<String>,
}

#[derive(Template)]
#[template(path = "saved-questions-list.html")]
struct QuestionListTemplate {
    poll_id: String,
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

    let result = save_question(&client, save_question, &"polls".to_string()).await?;

    log::info!("Result: {:?}", result);

    // Fetch the poll data from DynamoDB
    let get_item_input = GetItemInput::builder()
        .table_name("polls")
        .key("pollId", AttributeValue::S(poll_id.clone()))
        .build();

    let get_item_output = client.get_item(get_item_input).await?;

    // Extract the questions from the response
    let questions = if let Some(item) = get_item_output.item {
        let questions_list = item
            .get("questions")
            .and_then(|v| v.as_l())
            .unwrap_or_default();
        questions_list
            .iter()
            .filter_map(|q| {
                q.as_m()
                    .and_then(|m| m.get("question").and_then(|v| v.as_s()))
            })
            .cloned()
            .collect()
    } else {
        vec![]
    };

    let template = QuestionListTemplate { poll_id, questions };

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
