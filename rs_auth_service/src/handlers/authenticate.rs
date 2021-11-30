use actix_http::http::uri;
use actix_web::{web, HttpResponse};
use bson::oid::ObjectId;

use chrono::Utc;
use serde_qs;
use std::env;

use crate::db;
use crate::error::CustomError;
use crate::helpers::{http::request, token};
use crate::models::app_state::AppState;
use crate::models::params::{AuthCodeResponse, FormParams};
use crate::models::token::Token;
use crate::models::user::User;

pub async fn authenticate(
  form: web::Form<FormParams>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();

  let data = web::Json(User {
    _id: ObjectId::new(),
    username: form.username.clone(),
    email: "".to_owned(),
    password: form.password.clone(),
  });

  let user = request::<User, User>(&auth_endpoint, data)
    .await
    .map_err(|_e| CustomError::NotFound)?;

  let access_code = token::generate_access_code(
    &[
      form.client_id.clone(),
      form.redirect_url.clone(),
      Utc::now().to_string(),
    ]
    .concat(),
  );

  let _insert_result = db::token::insert(
    Token {
      access_code: access_code.clone(),
      username: user.username,
      user_id: user._id,
      client_id: form.client_id.clone(),
      expires: Utc::now().to_rfc2822(), // TODO add LIFE_SPAN
    },
    &state,
  )
  .await
  .unwrap();

  let response = AuthCodeResponse {
    access_code: access_code,
    state: form.state.clone(),
  };

  let query = serde_qs::to_string(&response);
  let redirect_url = [form.redirect_url.clone(), query.unwrap()].concat();
  let uri = uri::Builder::new()
    .path_and_query(redirect_url)
    .build()
    .unwrap();

  let response = HttpResponse::Found()
    .header("location", uri.to_string())
    .finish();
  Ok(response)
}
