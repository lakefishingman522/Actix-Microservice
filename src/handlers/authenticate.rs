use crate::error::map_io_error;
use crate::error::CustomError;
use crate::jwt;
use crate::state::AppState;
use crate::user::User;
use actix_http::cookie::Cookie;
use actix_web::{
  dev::HttpResponseBuilder, error, error::ResponseError, get, http::header, http::StatusCode, web,
  HttpMessage, HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum UserError {
  #[display(fmt = "An internal error occurred. Please try again later.")]
  InternalError,
}

impl error::ResponseError for UserError {
  fn error_response(&self) -> HttpResponse {
    HttpResponseBuilder::new(self.status_code())
      .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
      .body(self.to_string())
  }
  fn status_code(&self) -> StatusCode {
    match *self {
      UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

pub async fn authenticate(
  req: HttpRequest,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let db = &state.db;

  let cookie = req
    .cookie("cookie_test")
    .ok_or(CustomError::NoCookie)
    .map_err(|_e| CustomError::NoCookie)?;

  let token = cookie.value();
  let claims = jwt::verify_token(&token).map_err(|_e| CustomError::WrongToken)?;
  let uuid = Uuid::parse_str(&claims["_id"]).unwrap();
  let user = db
    .find_one::<User>(&uuid)
    .await
    .map_err(|_e| CustomError::NoUserFound)?;
  println!("222222{:?}", user.data.email);

  //let cookie = get_cookie_string_from_header(request);
  Ok(HttpResponse::Ok().body(""))
}
