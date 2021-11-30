use actix_web::{web, HttpResponse};
use askama_actix::Template;

use crate::error::CustomError;
use crate::models::params::Params;

#[derive(Template)]
#[template(path = "login.html")]

pub struct AuthFormTemplate<'a> {
  response_type: &'a str,
  client_id: &'a str,
  redirect_url: &'a str,
  state: &'a str,
}

pub async fn login(query: web::Query<Params>) -> Result<HttpResponse, CustomError> {
  Ok(
    HttpResponse::Ok().body(
      AuthFormTemplate {
        response_type: &query.response_type,
        client_id: &query.client_id,
        redirect_url: &query.redirect_url,
        state: &query.state,
      }
      .render()
      .unwrap(),
    ),
  )
}
