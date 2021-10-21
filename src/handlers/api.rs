use crate::error::CustomError;
use crate::models::{AccessCode, IdToken};
use crate::state::AppState;
use actix_web::{web, HttpRequest, HttpResponse};
use mongodb::bson::doc;

pub async fn api(
  req: HttpRequest,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let db = &state.db.database("auth-db");
  let auth_header = req.headers().get("authorization").unwrap().to_str().ok();

  println!("header {:?}", auth_header);

  // let access_code = &db
  //   .collection::<AccessCode>("tokens")
  //   .find_one(doc! {"access_code": &data.access_code}, None)
  //   .await
  //   .map_err(|_e| CustomError::Forbidden)?
  //   .unwrap();

  Ok(HttpResponse::Ok().body(format!("Welcome to API")))
}
