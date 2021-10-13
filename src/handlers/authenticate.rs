

use crate::error::CustomError;




use actix_web::{
  dev::HttpResponseBuilder, error, error::ResponseError, get, http::header, http::StatusCode, web,
  HttpMessage, HttpRequest, HttpResponse, Responder,
};
use askama_actix::{Template, TemplateIntoResponse};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
  response_type: String,
  client_id: String,
  redirect_url: String,
}

#[derive(Template)]
#[template(path = "signin.html")] // using the template in this path, relative
                                  // to the `templates` dir in the crate root
pub struct HelloTemplate<'a> {
  response_type: &'a str,
  client_id: &'a str,
  redirect_url: &'a str,
}

// pub async fn authenticate(
//   req: HttpRequest,
//   state: web::Data<AppState>,
// ) -> Result<HttpResponse, CustomError> {
//   let db = &state.db;

//   let cookie = req
//     .cookie("cookie_test")
//     .ok_or(CustomError::NoCookie)
//     .map_err(|_e| CustomError::NoCookie)?;

//   let token = cookie.value();
//   let claims = jwt::verify_token(&token).map_err(|_e| CustomError::WrongToken)?;
//   let uuid = Uuid::parse_str(&claims["_id"]).map_err(|_e| CustomError::WrongToken)?;
//   let user = db
//     .find_one::<User>(&uuid)
//     .await
//     .map_err(|_e| CustomError::NoUserFound)?;

//   //let cookie = get_cookie_string_from_header(request);
//   Ok(HttpResponse::Ok().body(""))
// }

pub async fn auth(query: web::Query<Params>) -> Result<HttpResponse, CustomError> {
  Ok(
    HttpResponse::Ok().body(
      HelloTemplate {
        response_type: &query.response_type,
        client_id: &query.client_id,
        redirect_url: &query.redirect_url,
      }
      .render()
      .unwrap(),
    ),
  )
}
