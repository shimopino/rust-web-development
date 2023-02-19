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

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "id: {}", self.0)
    }
}

#[tokio::main]
async fn main() {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );
    println!("question sample {:#?}", question);

    let updated_question = question.update_title("Refined First Question".to_string());
    println!("updated question sample {:#?}", updated_question);

    println!("Question ID => {}", updated_question.id);

    let hello = warp::get().map(|| format!("Hello, World!"));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}

#[test]
fn test_question_id() {
    let question_id = QuestionId::from_str("123").unwrap();

    // QuestionId のままだと等価比較のトレイトを実装していないのでエラーになる
    assert_eq!(question_id.0, "123".to_string());
}

#[test]
fn test_invalid_question_id() {
    let empty_question_id = QuestionId::from_str("");

    assert_eq!(empty_question_id.is_err(), true);
}
