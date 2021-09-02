use crate::state::AppState;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  uuid: String,
  email: String,
  password: String,
}

pub async fn auth() -> impl Responder {
  let user = User {
    uuid: "25CUZ20".to_owned(),
    email: "magazpablo@gmail.com".to_owned(),
    password: "11111".to_owned(),
  };

  HttpResponse::Ok().json(user)
}
