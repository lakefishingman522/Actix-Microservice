use actix_web::{web, HttpResponse, Responder};
use bson;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::bson::doc;
use std::env;

use crate::error::CustomError;
use crate::models::{
  AccessCode, AccessToken, IdToken, TokenPassReq, TokenReq, TokenResponse, User,
};
use crate::request::request;
use crate::state::AppState;
use crate::token::{generate_access_token, generate_id_token};

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
    .map_err(|_e| CustomError::NotFound)?;

  println!("[Token Pass] User: {:?}", user.clone());
  // FIXME independet endpoint

  let private_key = &state.private_key;

  let access_token = generate_access_token(
    &private_key,
    AccessToken {
      iss: env::var("ISSUER").unwrap(),
      exp: Utc::now().to_string(),
      sub: user._id.to_string(),
      azp: data.client_id.clone(),
    },
  );

  let id_token = generate_id_token(
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

  println!("[Token Pass] Token: {:?}", response.clone());

  Ok(HttpResponse::Ok().json(response))
}
