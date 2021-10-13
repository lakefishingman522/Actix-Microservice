use actix_http::cookie::Cookie;

use actix_web::{cookie, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use std::io::Result;



pub fn create_cookie(name: String, value: String) -> Cookie<'static> {
  Cookie::build(name, value)
    .path("/")
    //.secure(true)
    .http_only(true)
    .finish()
}

pub fn get_cookie(name: &str, req: HttpRequest) -> Result<Option<Cookie>> {
  Ok(req.cookie(name))
}

pub fn get_cookie_value(cookie: Cookie) -> String {
  cookie.value().to_owned()
}
