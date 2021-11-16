use crate::error::CustomError;
use crate::models::{AccessCode, TokenReq, TokenResponse};
use actix_web::{web, HttpResponse};
use std::env;

use crate::cookie;
use crate::request::request;

pub async fn callback(query: web::Query<AccessCode>) -> Result<HttpResponse, CustomError> {
  let token_endpoint = env::var("TOKEN_ENDPOINT").unwrap();

  let data = web::Json(TokenReq {
    grant_type: "authorization_code".to_owned(),
    access_code: query.access_code.clone(),
    redirect_url: env::var("REDIRECT_URL").unwrap(),
  });

  println!("Request to token with {:?}", data.clone());

  let token_response = request::<TokenReq, TokenResponse>(&token_endpoint, data)
    .await
    .unwrap();

  let access_cookie = cookie::create_cookie(
    "access_token".to_owned(),
    token_response.access_token.clone(),
  );
  let id_cookie = cookie::create_cookie("id_token".to_owned(), token_response.access_token.clone());

  let mut response = HttpResponse::Ok().json(token_response);
  response.add_cookie(&access_cookie);
  response.add_cookie(&id_cookie);
  Ok(response)
}
