use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::doc;
use std::env;

use crate::db;
use crate::error::CustomError;
use crate::helpers::{http::request, token};
use crate::models::app_state::AppState;
use crate::models::token::{AccessToken, IdToken, TokenReq, TokenResponse};
use crate::models::user::{SearchUser, User};

pub async fn token(
  data: web::Json<TokenReq>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let access_code = db::token::find_access_code(doc! {"access_code": &data.access_code}, &state)
    .await
    .map_err(|_e| CustomError::Forbidden)?
    .unwrap();

  let data = web::Json(SearchUser {
    username: access_code.username.clone(),
  });

  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();
  let user = request::<SearchUser, User>(&auth_endpoint, data)
    .await
    .map_err(|_e| CustomError::NotFound)
    .unwrap();

  let private_key = &state.private_key;

  let access_token = token::generate_access_token(
    &private_key,
    AccessToken {
      iss: env::var("ISSUER").unwrap(),
      exp: Utc::now().to_string(),
      sub: user._id.to_string(),
      azp: access_code.client_id.clone(),
    },
  );

  let id_token = token::generate_id_token(
    &private_key,
    IdToken {
      aud: access_code.client_id.clone(),
      azp: access_code.client_id.clone(),
      at_hash: user._id.to_string(),
      sub: user._id.to_string(),
      email: access_code.username.clone(),
    },
  );

  let response = TokenResponse {
    access_token: access_token,
    id_token: id_token,
    token_type: "".to_owned(),
    expires_in: env::var("JWT_LIFE_SPAN").unwrap(),
    scope: "".to_owned(),
    refresh_token: "".to_owned(),
  };
  Ok(HttpResponse::Ok().json(response))
}
