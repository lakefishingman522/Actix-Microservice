use crate::jwt;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder};

use crate::user::User;

pub async fn auth() -> impl Responder {
  let user = User {
    uuid: "25CUZ20".to_owned(),
    email: "magazpablo@gmail.com".to_owned(),
    password: "11111".to_owned(),
  };

  let token = jwt::generate_token(user);

  HttpResponse::Ok()
    //.json(user)
    .body(token)
}
