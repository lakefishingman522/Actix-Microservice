use actix_web::{web, HttpResponse};
use bson;
use bson::oid::ObjectId;
use chrono::Utc;
use std::env;

use crate::db;
use crate::error::CustomError;
use crate::helpers::{http::request, token};
use crate::models::app_state::AppState;
use crate::models::token::{AccessToken, IdToken, TokenPassReq, TokenResponse};
use crate::models::{token::Token, user::User};

pub async fn token_pass(
  data: web::Form<TokenPassReq>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  println!("[Token Pass]");
  let user_data = web::Json(User {
    _id: ObjectId::new(),
    username: data.username.clone(),
    email: "".to_owned(),
    password: data.password.clone(),
  });

  let user = request::<User, User>(&env::var("IDENTITY_ENDPOINT").unwrap(), user_data)
    .await
    .unwrap();

  let private_key = &state.private_key;

  let _insert_result = db::token::insert(
    Token {
      access_code: String::from(""),
      username: user.username,
      user_id: user._id,
      client_id: data.client_id.clone(),
      expires: Utc::now().to_rfc2822(), // TODO add LIFE_SPAN
    },
    &state,
  )
  .await
  .unwrap();

  let access_token = token::generate_access_token(
    &private_key,
    AccessToken {
      iss: env::var("ISSUER").unwrap(),
      exp: Utc::now().to_string(),
      sub: user._id.to_string(),
      azp: data.client_id.clone(),
    },
  );

  let id_token = token::generate_id_token(
    &private_key,
    IdToken {
      aud: data.client_id.clone(),
      azp: data.client_id.clone(),
      at_hash: user._id.to_string(),
      sub: user._id.to_string(),
      email: data.username.clone(),
    },
  );

  let response = TokenResponse {
    access_token: access_token.clone(),
    id_token: id_token.clone(),
    token_type: "Bearer".to_owned(),
    expires_in: env::var("JWT_LIFE_SPAN").unwrap(),
    scope: "".to_owned(),
    refresh_token: "".to_owned(),
  };

  Ok(HttpResponse::Ok().json(response))
}
