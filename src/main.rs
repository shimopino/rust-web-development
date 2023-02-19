use serde::Serialize;
use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};
use warp::Filter;

#[derive(Debug, Serialize)]
struct QuestionId(String);

#[derive(Debug, Serialize)]
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

async fn get_questions() -> Result<impl warp::Reply, warp::Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    Ok(warp::reply::json(&question))
}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use crate::{Question, QuestionId};
    use std::str::FromStr;

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

    #[test]
    fn test_question() {
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
    }
}
