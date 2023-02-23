use tracing::{event, Level};
use warp::http::StatusCode;

use crate::{store::Store, types::answer::NewAnswer};

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "rust-web-development", Level::INFO, "adding answers");

    match store.add_answer(new_answer).await {
        Ok(_) => {
            Ok(warp::reply::with_status("Answer Added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
