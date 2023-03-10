use std::collections::HashMap;
use tracing::{event, instrument, Level};
use warp::http::StatusCode;

use crate::{
    profanity::check_profanity,
    store::Store,
    types::{account::Session, answer::Answer},
};

#[instrument]
pub async fn add_answer(
    session: Session,
    store: Store,
    params: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let content = match check_profanity(params.get("content").unwrap().to_string()).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    let answer = Answer {
        content,
        question_id: params.get("questionId").unwrap().parse().unwrap(),
    };

    match store.add_answer(answer, account_id).await {
        Ok(_) => {
            event!(target: "warp-rest-api", Level::INFO, "POST NEW Answer");
            Ok(warp::reply::with_status("Answer added", StatusCode::OK))
        }
        Err(e) => Err(warp::reject::custom(e)),
    }
}
