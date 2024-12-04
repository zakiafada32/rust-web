use crate::store::Store;
use crate::types::answer::NewAnswer;
use warp::http::StatusCode;

pub async fn add_answer(
    store: Store,
    new_answer: NewAnswer,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_answer(new_answer).await {
        Ok(res) => Ok(warp::reply::with_status(
            warp::reply::json(&res),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
