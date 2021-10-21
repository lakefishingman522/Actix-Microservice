use actix_web::{dev::ServiceRequest, get, web, App, Error, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

use crate::state::AppState;
use crate::token::decode_token;

pub async fn bearer_auth_validator(
  req: ServiceRequest,
  credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
  let config = req
    .app_data::<Config>()
    .map(|data| data.clone())
    .unwrap_or_else(Default::default);

  println!("[AUTH] :{:?}", credentials.token());

  match validate_token(credentials.token()) {
    Ok(res) => {
      if res == true {
        Ok(req)
      } else {
        Err(AuthenticationError::from(config).into())
      }
    }
    Err(_) => Err(AuthenticationError::from(config).into()),
  }
}

fn validate_token(token: &str) -> Result<bool, std::io::Error> {
  let claims = decode_token(token).unwrap();
  let sub = &claims["sub"];
  println!("[AUTH] :{:?}", claims["sub"]);
  //FIXME db check
  if sub.len() > 20 {
    return Ok(true);
  }
  return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    "Authentication failed!",
  ));
}
