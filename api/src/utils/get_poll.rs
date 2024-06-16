use aws_sdk_dynamodb::{types::AttributeValue, Client};
use lambda_http::Error;

use crate::models::poll::{Poll, Question};

pub async fn get_poll(client: &Client, poll_id: &String, table: &String) -> Result<Poll, Error> {
    let request = client
        .get_item()
        .table_name(table)
        .key("pollId", AttributeValue::S(poll_id.to_string()));

    let response = request.send().await?;

    if let Some(item) = response.item {
        let poll_id = item.get("pollId").unwrap().as_s().unwrap().to_string();
        let title = item.get("title").unwrap().as_s().unwrap().to_string();
        let description = item.get("description").unwrap().as_s().unwrap().to_string();

        let questions = item.get("questions").unwrap().as_l().unwrap();
        let questions: Vec<Question> = questions
            .into_iter()
            .map(|attr| {
                let question = attr.as_m().unwrap();
                let question = Question {
                    question: question
                        .get("question")
                        .unwrap()
                        .as_s()
                        .unwrap()
                        .to_string(),
                    alternatives: question
                        .get("alternatives")
                        .unwrap()
                        .as_l()
                        .unwrap()
                        .into_iter()
                        .map(|attr| attr.as_s().unwrap().to_string())
                        .collect(),
                };
                question
            })
            .collect();

        let poll = Poll {
            poll_id,
            title,
            description,
            questions,
        };
        Ok(poll)
    } else {
        Err(Error::from("Poll not found".to_string()))
    }
}
