use crate::cookie;
use crate::jwt;
use actix_web::HttpMessage;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Response {
  _id: reddb::Uuid,
  //email: String,
  token: String,
}

pub async fn authenticate(req: HttpRequest) -> impl Responder {
  let cookie = cookie::get_cookie("cookie_test", req).unwrap();
  let token = cookie::get_cookie_value(cookie);
  let claims = jwt::verify_token(&token);
  println!("1111111{:?}", claims["email"]);

  //let cookie = get_cookie_string_from_header(request);
  HttpResponse::Ok()
}
