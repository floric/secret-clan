use serde::Serialize;
use std::convert::Infallible;
use warp::{
    hyper::StatusCode,
    reply::{Json, WithStatus},
    Rejection, Reply,
};

#[derive(Serialize)]
struct ErrorMessage {
    code: u16,
    message: String,
}

fn build_error_content(status: &StatusCode) -> ErrorMessage {
    ErrorMessage {
        code: status.as_u16(),
        message: String::from(status.canonical_reason().unwrap_or("unknown")),
    }
}

pub fn reply_with_error(status: StatusCode) -> WithStatus<Json> {
    warp::reply::with_status(warp::reply::json(&build_error_content(&status)), status)
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        return Ok(reply_with_error(StatusCode::NOT_FOUND));
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        return Ok(reply_with_error(StatusCode::METHOD_NOT_ALLOWED));
    }

    return Ok(reply_with_error(StatusCode::INTERNAL_SERVER_ERROR));
}
