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
    details: String,
}

#[derive(Serialize)]
struct SuccessMessage {
    code: u16,
    message: String,
}

pub fn reply_success(status: StatusCode) -> WithStatus<Json> {
    warp::reply::with_status(warp::reply::json(&build_success_content(&status)), status)
}

pub fn reply_error(status: StatusCode) -> WithStatus<Json> {
    warp::reply::with_status(warp::reply::json(&build_error_content(&status, "")), status)
}

pub fn reply_error_with_details(status: StatusCode, details: &str) -> WithStatus<Json> {
    warp::reply::with_status(
        warp::reply::json(&build_error_content(&status, details)),
        status,
    )
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        return Ok(reply_error_with_details(
            StatusCode::NOT_FOUND,
            "Path unsupported",
        ));
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        return Ok(reply_error_with_details(
            StatusCode::METHOD_NOT_ALLOWED,
            "Request not supported, check path and sent headers",
        ));
    }

    return Ok(reply_error_with_details(
        StatusCode::INTERNAL_SERVER_ERROR,
        "An internal error has happend, please contact the developers",
    ));
}

fn build_error_content(status: &StatusCode, details: &str) -> ErrorMessage {
    ErrorMessage {
        code: status.as_u16(),
        message: String::from(status.canonical_reason().unwrap_or("unknown")),
        details: String::from(details),
    }
}

fn build_success_content(status: &StatusCode) -> SuccessMessage {
    SuccessMessage {
        code: status.as_u16(),
        message: String::from(status.canonical_reason().unwrap_or("unknown")),
    }
}
