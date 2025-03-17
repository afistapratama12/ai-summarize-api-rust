use std::convert::Infallible;
use sea_orm::DatabaseConnection;
use warp::{self, Filter};

use crate::handler::{auth, file_handler};
use crate::common::error;
use crate::middleware::with_auth;

pub fn routes(db: DatabaseConnection) -> impl Filter<Extract = impl warp::Reply> + Clone {
  // create_customer(db.clone())
  //  .or(update_customer(db.clone()))
  //  .or(delete_customer(db))
  register(db.clone())
    .or(login(db.clone()))
    .or(upload_file(db.clone()))
    .or(summarize_file(db))
    .recover(error::handle_rejection)
}

// POST /auth/login
fn login(db: DatabaseConnection) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  println!("POST /auth/login");

  warp::path!("auth" / "login")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(auth::login)
}

// POST /auth/register
fn register(db: DatabaseConnection) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  println!("POST /auth/register");

  warp::path!("auth" / "register")
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(auth::register)
}

pub fn upload_file(db: DatabaseConnection) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  println!("POST /file/upload");

  warp::path!("file" / "upload")
    .and(with_auth())
    .and(warp::post())
    .and(warp::multipart::form().max_length(10_000_000))
    .and(with_db(db))
    .and_then(file_handler::upload_file_handler)
}

pub fn summarize_file(db: DatabaseConnection) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  println!("POST /file/summarize");

  warp::path!("file" / "summarize")
    .and(with_auth())
    .and(warp::post())
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(file_handler::summarize)
}

fn with_db(db: DatabaseConnection) -> impl Filter<Extract = (DatabaseConnection,), Error = Infallible> + Clone {
  warp::any().map(move || db.clone())
}
