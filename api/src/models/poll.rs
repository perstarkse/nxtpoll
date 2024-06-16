pub struct Poll {
    pub poll_id: String,
    pub title: String,
    pub description: String,
    pub questions: Vec<Question>,
}

pub struct Question {
    pub question: String,
    pub alternatives: Vec<String>,
}
