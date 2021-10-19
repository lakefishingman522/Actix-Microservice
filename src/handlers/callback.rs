use actix_web::{get, web, HttpResponse};
use askama_actix::Template;

use crate::error::CustomError;
use crate::models::SignInResponse;

#[derive(Template)]
#[template(path = "local.html")]

pub struct LocalTemplate<'a> {
  access_token: &'a str,
  id_token: &'a str,
  token_type: &'a str,
  expires_in: &'a str,
}

pub async fn callback(query: web::Query<SignInResponse>) -> Result<HttpResponse, CustomError> {
  Ok(
    HttpResponse::Ok().body(
      LocalTemplate {
        access_token: &query.access_token,
        id_token: &query.id_token,
        token_type: &query.token_type,
        expires_in: &query.expires_in,
      }
      .render()
      .unwrap(),
    ),
  )
}
