use actix_web::{web, HttpRequest, HttpResponse};

use crate::error::CustomError;
use crate::models::app_state::AppState;

pub async fn api(
  req: HttpRequest,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let _db = &state.db.database("auth-db");
  let _private_key = &state.private_key;

  let auth_header = req.headers().get("authorization").unwrap().to_str().ok();
  let access_token = auth_header
    .ok_or(CustomError::InvalidToken)
    .map_err(|_e| CustomError::InvalidToken)
    .unwrap();
  println!("header {:?}", access_token);

  // let access_code = &db
  //   .collection::<AccessCode>("tokens")
  //   .find_one(doc! {"access_code": &data.access_code}, None)
  //   .await
  //   .map_err(|_e| CustomError::Forbidden)?
  //   .unwrap();

  Ok(HttpResponse::Ok().body(format!("Welcome to API")))
}
