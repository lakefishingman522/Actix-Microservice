use actix_http::http::uri;
use actix_web::{web, HttpResponse};
use bson;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::bson::doc;
use serde_qs;
use std::env;

use crate::cookie;
use crate::error::CustomError;
use crate::models::{AuthCodeResponse, FormParams, User};
use crate::request::request;
use crate::state::AppState;
use crate::token;

pub async fn login(
  form: web::Form<FormParams>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();

  let user_json = web::Json(User {
    _id: ObjectId::new(),
    username: form.username.clone(),
    email: "".to_owned(),
    password: form.password.clone(),
  });

  let user = request::<User>(&auth_endpoint, user_json)
    .await
    .map_err(|_e| CustomError::NotFound)?;

  let access_code =
    token::generate_access_code(&[form.client_id.clone(), form.redirect_url.clone()].concat());

  let db = &state.db.database("auth-db").collection("tokens");

  let _insert_result = db
    .insert_one(
      doc! {
         "access_code": access_code.clone(),
         "expires": &Utc::now().to_rfc2822()
      },
      None,
    )
    .await
    .map_err(|_e| CustomError::Unknown)
    .unwrap();

  //println!("{:?}", access_code);
  // let payload = AccessToken {
  //   iss: env::var("ISSUER").unwrap(),
  //   exp: Utc::now().to_string(),
  // };

  // let access_token = token::generate_access_token(
  //   &private_key,
  //   AccessToken {
  //     iss: env::var("ISSUER").unwrap(),
  //     exp: Utc::now().to_string(),
  //   },
  // );

  // let id_token = token::generate_id_token(
  //   &private_key,
  //   IdToken {
  //     aud: form.client_id.clone(),
  //     azp: form.client_id.clone(),
  //     at_hash: user._id.to_string(),
  //     sub: user._id.to_string(),
  //     email: user.username,
  //   },
  // );

  // let response = SignInResponse {
  //   access_token: access_token.clone(),
  //   id_token: id_token.clone(),
  //   token_type: "JWT".to_owned(),
  //   expires_in: env::var("JWT_LIFE_SPAN").unwrap(),
  //   scope: "".to_owned(),
  //   refresh_token: "".to_owned(),
  // };

  // let access_cookie = cookie::create_cookie("access_token".to_owned(), access_token);
  // let id_cookie = cookie::create_cookie("id_token".to_owned(), id_token);

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
  // response.add_cookie(&access_cookie);
  // response.add_cookie(&id_cookie);
  Ok(response)
}
