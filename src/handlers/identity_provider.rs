use actix_web::{web, HttpResponse};

use mongodb::{
  bson::{doc, Document},
  error::Error,
  options::FindOptions,
};

use crate::error::CustomError;
use crate::models::User;
use crate::state::AppState;

pub async fn find_user(
  data: web::Json<User>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let db = &state.db.database("auth-db").collection::<User>("users");
  println!("[Identity] Username: {:?}", &data.username);

  let user = db
    .find_one(doc! {"username": &data.username}, None)
    .await
    .map_err(|_e| CustomError::Forbidden)
    .unwrap();

  println!("User: {:?}", user);

  Ok(HttpResponse::Ok().json(user))
}
