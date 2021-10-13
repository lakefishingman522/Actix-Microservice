
use actix_web::{web, HttpResponse};



use crate::error::CustomError;
use crate::request::request;
use crate::state::AppState;
use crate::user::User;
use reddb::Document;
use serde::{Deserialize, Serialize};
use std::env;
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
struct Response {
  _id: reddb::Uuid,
  //email: String,
  token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormParams {
  username: String,
  password: String,
  redirect_url: String,
}

pub async fn signin(
  form: web::Form<FormParams>,
  _state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();
  let user_json = web::Json(User {
    username: form.username.clone(),
    password: form.password.clone(),
  });
  let doc = request::<User, Document<User>>(&auth_endpoint, user_json)
    .await
    //.ok_or(CustomError::NotFound)
    .map_err(|_e| CustomError::NotFound)?;

  //println!("aaaaaa{:?}", doc);

  // let token = jwt::generate_token(doc.clone());
  // let cookie = cookie::create_cookie("cookie_test".to_owned(), token.clone());

  // let mut response = HttpResponse::Ok().json(Response {
  //   _id: doc._id,
  //   token: token,
  // });

  //response.add_cookie(&cookie);
  // response
  Ok(HttpResponse::Ok().json(doc))
}
