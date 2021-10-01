use crate::jwt;
use crate::state::AppState;
use actix_web::{HttpResponse, Responder};
use std::env;

use crate::client::request;
use crate::user::User;

pub async fn auth() -> impl Responder {
  let endpoint = env::var("IDENTITY_ENDPOINT").unwrap();
  let user = request::<User>(&endpoint).await;

  let token = jwt::generate_token(user.unwrap());

  HttpResponse::Ok()
    //.json(user)
    .body(token)
}
