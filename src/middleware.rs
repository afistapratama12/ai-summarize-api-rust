use warp::{Filter, Rejection, reject::custom};

use crate::libs::jwt::validate_jwt;
use crate::libs::jwt::Claims;

#[derive(Debug)]
pub struct Unauthorized;

impl warp::reject::Reject for Unauthorized {}

pub fn with_auth() -> impl Filter<Extract = (Claims,), Error = Rejection> + Clone {
  warp::header::optional::<String>("Authorization")
    .and_then(|auth_header: Option<String>| async move {
      if let Some(header_value) = auth_header {
        if let Some(token) = header_value.strip_prefix("Bearer ") {
          match validate_jwt(token) {
            Ok(token_data) => Ok(token_data.claims),
            Err(_) => Err(custom(Unauthorized)),
          }
        } else {
          Err(custom(Unauthorized))
        }
      } else {
          Err(custom(Unauthorized))
      }
    })
}