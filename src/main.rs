use serde::Serialize;
use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
    str::FromStr,
};
use warp::{
    cors::CorsForbidden, http::Method, http::StatusCode, reject::Reject, Filter, Rejection, Reply,
};

struct Store {
    questions: HashMap<QuestionId, Question>,
}

impl Store {
    fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    fn add_question(mut self, question: Question) -> Self {
        self.questions.insert(question.id.clone(), question);

        self
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash)]
struct QuestionId(String);

#[derive(Debug, Serialize, Clone, PartialEq, Hash)]
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

    match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(_InvalidId) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "No valid ID presented".to_string(),
            StatusCode::NOT_FOUND,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}

#[derive(Debug)]
struct InvalidId;
impl Reject for InvalidId {}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items.with(cors);

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
