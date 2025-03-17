// external module
use std::convert::Infallible;
use warp::{
  reject::Reject,
  Rejection, 
  Reply, 
  http::StatusCode,
};

use crate::middleware::Unauthorized;
use crate::handler::response::error_resp;

#[derive(Debug)]
pub struct InternalServerError {
  pub message: String,
}

impl Reject for InternalServerError {}

// handle rejection
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
  let code;
  let message;

  if err.is_not_found() {
    code = StatusCode::NOT_FOUND;
    message = "Not Found";
  } else if let Some(InternalServerError{message: msg}) = err.find() {
    code = StatusCode::INTERNAL_SERVER_ERROR;
    message = msg;
  } else if let Some(Unauthorized) = err.find() {
    code = StatusCode::UNAUTHORIZED;
    message = "Unauthorized";
  } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
    code = StatusCode::METHOD_NOT_ALLOWED;
    message = "Method Not Allowed";
  } else {
    println!("unhandled error: {:?}", err);
    code = StatusCode::INTERNAL_SERVER_ERROR;
    message = "Internal Server Error";
  }

  Ok(error_resp(message, code))
}