use warp::{http::StatusCode, Rejection};
use sea_orm::{
  sqlx::types::chrono, 
  ActiveModelTrait, 
  ColumnTrait, 
  DatabaseConnection, 
  EntityTrait, 
  QueryFilter, 
  Set
};

use super::{
  response::{
    error_resp,
    created_resp,
    success_resp,
    RegisterResponse,
    LoginResponse
  }, 
  request::{LoginRequest, RegisterRequest} 
};
use crate::common::error::InternalServerError;
use crate::libs::{hash, jwt};
use crate::model::{self, users::{ActiveModel, Entity as User}};

pub async fn register(req: RegisterRequest, db: DatabaseConnection) -> Result<impl warp::Reply, Rejection> {
  // check email format
  if !req.email.contains("@") {
    return Ok(error_resp("Invalid email format", StatusCode::BAD_REQUEST));
  }

  let new_id = uuid::Uuid::new_v4().to_string();
  let date_now = chrono::Utc::now().naive_utc();

  let hash_password = hash::hash_password(req.password.clone().as_str());

  // prepare struct db
  let new_user = ActiveModel {
    id: Set(new_id.clone()),
    name: Set(req.name.clone()),
    email: Set(req.email.clone()),
    password: Set(hash_password),
    created_at: Set(date_now.clone()),
    ..Default::default()
  };

  // save to db
  new_user.insert(&db).await
  .map_err(|_| warp::reject::custom(InternalServerError{
    message: "Error creating user".to_string(),
  }))?;

  // create response struct
  let register_resp = RegisterResponse {
    name: req.name,
    email: req.email,
    message: "User created successfully".to_string(),
  };

  // return response
  Ok(created_resp(warp::reply::json(&register_resp)))
}


pub async fn login(req: LoginRequest, db: DatabaseConnection) -> Result<impl warp::Reply, Rejection> {
  if !req.email.contains("@") {
    return Ok(error_resp("Invalid email format", StatusCode::BAD_REQUEST));
  }

  // find user by email
  let user = User::find()
    .filter(model::users::Column::Email.contains(req.email.clone()))
    .one(&db)
    .await
    .map_err(|_| warp::reject::custom(InternalServerError{
      message: "Error finding user".to_string(),
    }))?
    .unwrap();

  // check if user is found
  if user.email.is_empty() || user.password.is_empty() {
    return Ok(error_resp("User not found", StatusCode::NOT_FOUND));
  }

  // check password
  if !hash::verify_password(req.password.as_str(), user.password.as_str()) {
    return Ok(error_resp("Invalid password", StatusCode::UNAUTHORIZED));
  }

  let token = jwt::generate_jwt(user.id.as_str(), &user.email);

  let login_resp = LoginResponse{
    name: user.name.clone(),
    email: req.email.clone(),
    token: token,
  };

  Ok(success_resp(warp::reply::json(&login_resp)))
}

