use actix_web::{web, HttpResponse};
use chrono::Utc;
use mongodb::bson::doc;
use std::env;

use crate::error::CustomError;
use crate::models::{AccessCode, AccessToken, IdToken, SearchUser, TokenReq, TokenResponse, User};
use crate::request::request;
use crate::state::AppState;
use crate::token::{generate_access_token, generate_id_token};

pub async fn token(
  data: web::Json<TokenReq>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let db = &state.db.database("auth-db");

  let access_code = &db
    .collection::<AccessCode>("tokens")
    .find_one(doc! {"access_code": &data.access_code}, None)
    .await
    .map_err(|_e| CustomError::Forbidden)?
    .unwrap();

  println!("[Token] Access Code: {:?}", access_code);
  let data = web::Json(SearchUser {
    username: access_code.username.clone(),
  });

  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();
  let user = request::<SearchUser, User>(&auth_endpoint, data)
    .await
    .map_err(|_e| CustomError::NotFound)
    .unwrap();
  println!("[Token] User: {:?}", user);

  let private_key = &state.private_key;

  let access_token = generate_access_token(
    &private_key,
    AccessToken {
      iss: env::var("ISSUER").unwrap(),
      exp: Utc::now().to_string(),
      sub: user._id.to_string(),
      azp: access_code.client_id.clone(),
    },
  );

  let id_token = generate_id_token(
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
