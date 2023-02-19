use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use warp::Filter;

#[derive(Debug)]
struct QuestionId(String);

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }

    fn update_title(self, new_title: String) -> Self {
        Question::new(self.id, new_title, self.content, self.tags)
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("question sample {:?}", question);

    let updated_question = question.update_title("Refined First Question".to_string());
    println!("updated question sample {:?}", updated_question);

    let hello = warp::get().map(|| format!("Hello, World!"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
