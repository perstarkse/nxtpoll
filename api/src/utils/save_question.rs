use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

#[derive(Debug, Clone)]
pub struct SaveQuestion {
    pub poll_id: String,
    pub question: String,
    pub alternatives: Vec<String>,
}

pub async fn save_question_fn(
    client: &Client,
    question: SaveQuestion,
    table: &String,
) -> Result<bool, Error> {
    let id_av = AttributeValue::S(question.poll_id.clone());
    let question_av = AttributeValue::S(question.question.clone());

    // Create an array of AttributeValue for alternatives
    let alternatives_av: Vec<AttributeValue> = question
        .alternatives
        .iter()
        .map(|alt| AttributeValue::S(alt.clone()))
        .collect();

    // Create an object for the question
    let question_object = AttributeValue::M(
        vec![
            ("question".to_string(), question_av),
            (
                "alternatives".to_string(),
                AttributeValue::L(alternatives_av),
            ),
        ]
        .into_iter()
        .collect(),
    );

    let question_list = AttributeValue::L(vec![question_object]);

    let request = client
        .update_item()
        .table_name(table)
        .key("pollId", id_av)
        .update_expression(
            "SET questions = list_append(if_not_exists(questions, :empty_list), :question)",
        )
        .expression_attribute_values(":empty_list", AttributeValue::L(vec![]))
        .expression_attribute_values(":question", question_list);

    let response = request.send().await;

    log::info!("Response: {:?}", response);

    match response {
        Ok(_) => Ok(true),
        Err(e) => Err(e.into()),
    }
}
