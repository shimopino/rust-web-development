use tracing::{event, Level};
use warp::http::StatusCode;

use crate::{
    profanity::check_profanity, store::Store, types::answer::NewAnswer,
};

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "rust-web-development", Level::INFO, "adding answers");

    let content = match check_profanity(new_answer.content).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let answer = NewAnswer {
        content,
        question_id: new_answer.question_id,
    };

    match store.add_answer(answer).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Answer Added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
