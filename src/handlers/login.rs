use crate::jwt;
use actix_web::{web, HttpResponse, Responder};

use reddb::Document;
use serde::{Deserialize, Serialize};
use std::env;

use crate::cookie;
use crate::request::request;
use crate::user::User;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Response {
  _id: reddb::Uuid,
  //email: String,
  token: String,
}

pub async fn login(req: web::Json<User>) -> impl Responder {
  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();
  let doc = request::<User, Document<User>>(&auth_endpoint, req)
    .await
    .unwrap();

  let token = jwt::generate_token(doc.data);
  let cookie = cookie::create_cookie("cookie_test".to_owned(), token.clone());

  let mut response = HttpResponse::Ok().json(Response {
    _id: doc._id,
    token: token,
  });

  response.add_cookie(&cookie);
  response
}
