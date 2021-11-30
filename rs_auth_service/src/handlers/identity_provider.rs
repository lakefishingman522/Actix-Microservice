use actix_web::{web, HttpResponse};

use mongodb::bson::doc;

use crate::db;
use crate::error::CustomError;
use crate::models::app_state::AppState;
use crate::models::user::User;

pub async fn find_user(
  data: web::Json<User>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let user = db::user::find_one(doc! {"username": &data.username}, &state)
    .await
    .map_err(|_e| CustomError::NotFound)
    .unwrap();

  println!("User: {:?}", user);

  Ok(HttpResponse::Ok().json(user))
}
