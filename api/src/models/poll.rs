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
use std::fmt;

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.question)
    }
}
