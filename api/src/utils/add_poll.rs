use aws_sdk_dynamodb::types::AttributeValue;
use aws_sdk_dynamodb::{Client, Error};

#[derive(Debug, Clone)]
pub struct Poll {
    pub poll_id: String,
    pub title: String,
    pub description: String,
}

pub async fn add_poll(client: &Client, poll: Poll, table: &String) -> Result<bool, Error> {
    let id_av = AttributeValue::S(poll.poll_id);
    let title_av = AttributeValue::S(poll.title);
    let description_av = AttributeValue::S(poll.description);

    let request = client
        .put_item()
        .table_name(table)
        .item("pollId", id_av)
        .item("title", title_av)
        .item("description", description_av);

    let response = request.send().await;

    log::info!("Response: {:?}", response);

    match response {
        Ok(_) => Ok(true),
        Err(e) => Err(e.into()),
    }
}
