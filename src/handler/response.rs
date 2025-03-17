use serde::Serialize;
use warp::{self, http::StatusCode};
use warp::reply::{WithStatus, Json, with_status, json};

#[derive(Clone, Debug, Serialize)]
pub struct RegisterResponse {
  pub name: String,
  pub email: String,
  pub message: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct LoginResponse {
  pub name: String,
  pub email: String,
  pub token: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct UploadResponse {
  pub file_id: String,
  pub file_ext: String,
  // pub extracted_text: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct SummarizeResponse {
  pub summary: String,
  pub content: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct Error {
  pub message: String,
  pub status: u16,
}

pub fn error_resp(msg: &str, status: StatusCode) -> WithStatus<Json> {
  with_status(
      json(&Error {
        message: msg.to_string(),
          status: status.as_u16(),
      }),
      status,
  )
}

pub fn success_resp(data: Json) -> WithStatus<Json> {
  with_status(
    data,
    StatusCode::OK,
  )
}

pub fn created_resp(data: Json) -> WithStatus<Json> {
  warp::reply::with_status(
    data,
    StatusCode::CREATED,
  )
}