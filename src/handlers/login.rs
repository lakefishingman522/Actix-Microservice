use actix_http::http::uri;
use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use reddb::Document;
use serde_qs;
use std::env;

use crate::cookie;
use crate::error::CustomError;
use crate::models::{AccessToken, FormParams, IdToken, SignInResponse, User};
use crate::request::request;
use crate::state::AppState;
use crate::token;

pub async fn login(
  form: web::Form<FormParams>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();
  let user_json = web::Json(User {
    _id: form.client_id.clone(),
    username: form.username.clone(),
    password: form.password.clone(),
  });
  let doc = request::<User, Document<User>>(&auth_endpoint, user_json)
    .await
    .map_err(|_e| CustomError::NotFound)?;

  let payload = AccessToken {
    iss: env::var("ISSUER").unwrap(),
    exp: Utc::now().to_string(),
  };

  let private_key = state.private_key.clone();
  let access_token = token::generate_token(
    &private_key,
    AccessToken {
      iss: env::var("ISSUER").unwrap(),
      exp: Utc::now().to_string(),
    },
  );

  let id_token = token::generate_token(
    &private_key,
    IdToken {
      _id: doc._id.to_string(),
      username: doc.data.username,
    },
  );

  let response = SignInResponse {
    access_token: access_token.clone(),
    id_token: id_token.clone(),
    token_type: "JWT".to_owned(),
    expires_in: env::var("JWT_LIFE_SPAN").unwrap(),
  };

  let query = serde_qs::to_string(&response);
  let redirect_url = [form.redirect_url.clone(), query.unwrap()].concat();
  let uri = uri::Builder::new()
    .path_and_query(redirect_url)
    .build()
    .unwrap();

  let access_cookie = cookie::create_cookie("access_token".to_owned(), access_token);
  let id_cookie = cookie::create_cookie("id_token".to_owned(), id_token);
  let mut response = HttpResponse::Found()
    .header("location", uri.to_string())
    .finish();
  response.add_cookie(&access_cookie);
  response.add_cookie(&id_cookie);
  Ok(response)
}
